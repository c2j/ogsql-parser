package io.github.c2j.ogsql;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Duration;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Throughput / soak sanity: many round trips through one child process must not
 * leak, crash, or corrupt the stream. Local pipe latency is ~1-2 ms per call, so
 * 2,000 iterations complete in a few seconds.
 */
class OgsqlStressTest {

    private static Ogsql ogsql;

    @BeforeAll
    static void start() {
        if (System.getProperty("ogsql.lib.path") == null) {
            for (String cand : new String[]{"../target/debug/ogsql", "../target/release/ogsql"}) {
                Path p = Paths.get(cand).toAbsolutePath().normalize();
                if (Files.exists(p)) {
                    System.setProperty("ogsql.lib.path", p.toString());
                    break;
                }
            }
        }
        ogsql = Ogsql.newInstance(Duration.ofSeconds(30), 3);
    }

    @AfterAll
    static void stop() {
        if (ogsql != null) {
            ogsql.close();
        }
    }

    @Test
    void two_thousand_round_trips_without_failure() {
        int n = 2000;
        for (int i = 0; i < n; i++) {
            ParseResult r = ogsql.parse("SELECT " + i + " AS n, 'x' AS s FROM t WHERE id = " + (i % 100));
            assertEquals(1, r.statementCount(), "iteration " + i);
            assertEquals(0, r.errorCount(), "iteration " + i);

            if (i % 50 == 0) {
                String f = ogsql.format("select " + i, FormatOptions.builder().keywordCase("upper").build());
                assertTrue(f.contains("SELECT " + i), f);
                assertTrue(ogsql.validate("SELECT " + i).valid());
            }
        }
        assertTrue(ogsql.isAlive(), "child must still be alive after " + n + " calls");
    }
}
