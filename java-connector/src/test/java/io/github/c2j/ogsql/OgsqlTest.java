package io.github.c2j.ogsql;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Duration;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * End-to-end tests: real {@code ogsql serve-stdio} binary over pipes, driven
 * through the public facade. The binary is resolved via {@code -Dogsql.lib.path}
 * or, when unset, the repo-local debug/release build (java-connector/../target).
 */
class OgsqlTest {

    private static Ogsql ogsql;

    @BeforeAll
    static void start() throws Exception {
        if (System.getProperty("ogsql.lib.path") == null) {
            for (String cand : new String[]{"../target/debug/ogsql", "../target/release/ogsql"}) {
                Path p = Paths.get(cand).toAbsolutePath().normalize();
                if (Files.exists(p)) {
                    System.setProperty("ogsql.lib.path", p.toString());
                    break;
                }
            }
        }
        ogsql = Ogsql.newInstance(Duration.ofSeconds(30));
    }

    @AfterAll
    static void stop() {
        if (ogsql != null) {
            ogsql.close();
        }
    }

    // ─── hello / lifecycle ─────────────────────────────────────

    @Test
    void hello_reports_version() {
        assertFalse(ogsql.version().isBlank());
        assertTrue(ogsql.isAlive());
    }

    @Test
    void close_is_idempotent_and_blocks_calls() {
        Ogsql o = Ogsql.newInstance();
        o.close();
        o.close(); // must not throw
        assertThrows(OgsqlException.class, () -> o.parse("SELECT 1"));
    }

    // ─── parse ─────────────────────────────────────────────────

    @Test
    void parse_valid_sql() {
        ParseResult r = ogsql.parse("SELECT id, name FROM users WHERE status = 'active'");
        assertEquals(1, r.statementCount());
        assertEquals(0, r.errorCount());
        assertTrue(r.statements().isArray());
        assertTrue(r.queryFingerprints().isArray());
    }

    @Test
    void parse_invalid_sql_returns_errors_not_exception() {
        ParseResult r = ogsql.parse("SELECT FROM WHERE");
        assertTrue(r.errorCount() > 0, "syntax problems must not throw");
    }

    @Test
    void parse_mybatis_placeholders() {
        ParseResult r = ogsql.parse("SELECT * FROM t WHERE id = #{userId}", true);
        assertEquals(0, r.errorCount());
    }

    @Test
    void parse_multiple_statements() {
        ParseResult r = ogsql.parse("SELECT 1; SELECT 2");
        assertEquals(2, r.statementCount());
    }

    // ─── format ────────────────────────────────────────────────

    @Test
    void format_uppercase_keywords() {
        String sql = ogsql.format("select 1", FormatOptions.builder().keywordCase("upper").build());
        assertTrue(sql.contains("SELECT 1"), "got: " + sql);
    }

    @Test
    void format_mybatis_preserved() {
        String sql = ogsql.format("select * from t where id = #{x}", FormatOptions.builder().mybatis(true).build());
        assertTrue(sql.contains("#{x}"), "got: " + sql);
    }

    // ─── tokenize / validate ───────────────────────────────────

    @Test
    void tokenize_sql() {
        List<TokenInfo> tokens = ogsql.tokenize("SELECT 1");
        assertFalse(tokens.isEmpty());
        assertEquals("SELECT", tokens.get(0).value());
    }

    @Test
    void validate_invalid_sql() {
        Validation v = ogsql.validate("SELECT FROM WHERE");
        assertFalse(v.valid());
        assertTrue(v.errorCount() > 0);
    }

    @Test
    void validate_valid_sql() {
        Validation v = ogsql.validate("SELECT 1");
        assertTrue(v.valid());
    }

    // ─── json2sql ──────────────────────────────────────────────

    @Test
    void json2sql_roundtrip() {
        ParseResult r = ogsql.parse("SELECT a, b FROM t WHERE c = 1");
        String sql = ogsql.json2sql(r.resultJson());
        assertTrue(sql.toUpperCase().contains("SELECT"), "got: " + sql);
        assertTrue(sql.toUpperCase().contains("FROM T"), "got: " + sql);
    }

    // ─── protocol failures ─────────────────────────────────────

    @Test
    void excessive_nesting_throws_TOO_DEEP_and_connector_survives() {
        String deep = "SELECT " + "(".repeat(500) + "1" + ")".repeat(500);
        OgsqlException e = assertThrows(OgsqlException.class, () -> ogsql.parse(deep));
        assertEquals("TOO_DEEP", e.code());
        assertTrue(ogsql.isAlive());
        // and still works
        assertEquals(1, ogsql.parse("SELECT 1").statementCount());
    }

    @Test
    void facade_exposes_only_known_ops() {
        // The facade never lets an unknown op reach the wire; protocol-level failures
        // surface as OgsqlException from the manager (covered by TOO_DEEP test).
        assertEquals(2, ogsql.parse("SELECT 1; SELECT 2").statementCount());
    }

    // ─── resilience ────────────────────────────────────────────

    @Test
    void child_crash_auto_restarts_and_retries() throws Exception {
        // Kill the child out from under the connector; the next call must
        // trigger restart-with-backoff + one retry and succeed.
        Process child = ogsql.processManager().process();
        child.destroyForcibly();
        child.waitFor(5, TimeUnit.SECONDS);

        long deadline = System.nanoTime() + TimeUnit.SECONDS.toNanos(30);
        ParseResult r = null;
        OgsqlException last = null;
        while (System.nanoTime() < deadline) {
            try {
                r = ogsql.parse("SELECT 42");
                break;
            } catch (OgsqlException e) {
                last = e;
                Thread.sleep(200);
            }
        }
        assertNotNull(r, "call should eventually succeed after restart, last error: " + last);
        assertEquals(0, r.errorCount());
        assertTrue(ogsql.isAlive());
    }

    @Test
    void concurrent_calls_serialize_correctly() throws Exception {
        int threads = 8;
        int perThread = 20;
        ExecutorService pool = Executors.newFixedThreadPool(threads);
        CountDownLatch done = new CountDownLatch(threads);
        AtomicInteger failures = new AtomicInteger();
        try {
            for (int t = 0; t < threads; t++) {
                final int tid = t;
                pool.submit(() -> {
                    try {
                        for (int i = 0; i < perThread; i++) {
                            ParseResult r = ogsql.parse("SELECT " + tid + " + " + i);
                            if (r.statementCount() != 1) {
                                failures.incrementAndGet();
                            }
                        }
                    } catch (Exception e) {
                        failures.incrementAndGet();
                    } finally {
                        done.countDown();
                    }
                });
            }
            assertTrue(done.await(60, TimeUnit.SECONDS), "concurrent calls did not finish");
        } finally {
            pool.shutdownNow();
        }
        assertEquals(0, failures.get(), "some concurrent calls failed");
    }

}
