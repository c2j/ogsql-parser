package io.github.c2j.ogsql;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

import java.io.IOException;
import java.nio.file.Path;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Java facade for ogsql-parser (openGauss/GaussDB SQL parsing) — usage experience
 * modelled after DuckDB's JDBC driver: one dependency, zero configuration.
 *
 * <pre>{@code
 * try (Ogsql ogsql = Ogsql.newInstance()) {
 *     ParseResult r = ogsql.parse("SELECT * FROM t WHERE id = #{id}", true);
 *     String formatted = ogsql.format("select 1", FormatOptions.builder().keywordCase("upper").build());
 *     Validation v = ogsql.validate("SELECT FROM WHERE");
 * }
 * }</pre>
 *
 * <p>Internally this spawns {@code ogsql serve-stdio} as a child process and
 * speaks the NDJSON line protocol over stdin/stdout (docs/stdio-protocol.md):
 * process isolation is preserved, so a parser crash can never take down the JVM —
 * the child is automatically restarted with backoff. The binary is resolved
 * DuckDB-style: {@code -Dogsql.lib.path} override, then a bundled platform binary
 * (see {@code src/main/resources/ogsql_<os>_<arch>}).
 */
public final class Ogsql implements AutoCloseable {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    private final OgsqlProcessManager pm;

    private Ogsql(OgsqlProcessManager pm) {
        this.pm = pm;
    }

    /**
     * Create a connector: locate the binary, spawn {@code serve-stdio}, and
     * handshake ({@code hello}).
     *
     * @throws OgsqlException when the binary cannot be located/started or the
     *                        protocol versions mismatch
     */
    public static Ogsql newInstance() throws OgsqlException {
        return newInstance(Duration.ofSeconds(30), 3);
    }

    public static Ogsql newInstance(Duration timeout) throws OgsqlException {
        return newInstance(timeout, 3);
    }

    /**
     * @param timeout      per-call response timeout
     * @param maxRestarts  automatic child-process restarts (with backoff) before
     *                     giving up; 0 disables restart
     */
    public static Ogsql newInstance(Duration timeout, int maxRestarts) throws OgsqlException {
        Path binary = NativeBinaryLoader.resolveBinary();
        try {
            return new Ogsql(new OgsqlProcessManager(binary, timeout.toMillis(), maxRestarts));
        } catch (IOException e) {
            throw new OgsqlException("failed to start ogsql serve-stdio from " + binary + ": " + e.getMessage(), e);
        }
    }

    // ─── parse ─────────────────────────────────────────────────

    public ParseResult parse(String sql) {
        return parse(sql, false, false);
    }

    /** @param mybatis preserve #{param} / ${expr} placeholders */
    public ParseResult parse(String sql, boolean mybatis) {
        return parse(sql, mybatis, false);
    }

    public ParseResult parse(String sql, boolean mybatis, boolean preserveComments) {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("sql", sql);
        req.put("mybatis", mybatis);
        req.put("preserve_comments", preserveComments);
        return ParseResult.from(pm.call("parse", req));
    }

    // ─── format ────────────────────────────────────────────────

    public String format(String sql) {
        return format(sql, FormatOptions.DEFAULT);
    }

    public String format(String sql, FormatOptions opts) {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("sql", sql);
        req.put("indent", opts.indent());
        req.put("keyword_case", opts.keywordCase());
        req.put("comma_style", opts.commaStyle());
        req.put("line_width", opts.lineWidth());
        req.put("uppercase", opts.uppercase());
        req.put("mybatis", opts.mybatis());
        req.put("no_select_newline", opts.noSelectNewline());
        req.put("no_logical_newline", opts.noLogicalNewline());
        req.put("no_semicolon_newline", opts.noSemicolonNewline());
        return pm.call("format", req).path("sql").asText();
    }

    // ─── tokenize ──────────────────────────────────────────────

    public List<TokenInfo> tokenize(String sql) {
        return tokenize(sql, false, false);
    }

    public List<TokenInfo> tokenize(String sql, boolean mybatis, boolean preserveComments) {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("sql", sql);
        req.put("mybatis", mybatis);
        req.put("preserve_comments", preserveComments);
        JsonNode result = pm.call("tokenize", req);
        List<TokenInfo> out = new ArrayList<>();
        for (JsonNode t : result.path("tokens")) {
            out.add(new TokenInfo(
                t.path("type").asText(),
                t.path("value").asText(),
                t.path("line").asInt(),
                t.path("column").asInt()));
        }
        return Collections.unmodifiableList(out);
    }

    // ─── validate ──────────────────────────────────────────────

    public Validation validate(String sql) {
        return validate(sql, false, false);
    }

    public Validation validate(String sql, boolean mybatis, boolean strict) {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("sql", sql);
        req.put("mybatis", mybatis);
        req.put("strict", strict);
        return Validation.from(pm.call("validate", req));
    }

    // ─── json2sql ──────────────────────────────────────────────

    /**
     * Convert a parse result back to SQL. Feed it {@link ParseResult#resultJson()}
     * or any object shaped like {@code {"statements": [...]}}.
     */
    public String json2sql(String astJson) {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("json", astJson);
        return pm.call("json2sql", req).path("sql").asText();
    }

    // ─── introspection / lifecycle ─────────────────────────────

    /** Rust crate version reported by the {@code hello} handshake. */
    public String version() {
        return pm.version();
    }

    public boolean isAlive() {
        return pm.isAlive();
    }

    /** Package-private: process manager (tests simulate child crashes). */
    OgsqlProcessManager processManager() {
        return pm;
    }

    @Override
    public void close() {
        pm.close();
    }
}
