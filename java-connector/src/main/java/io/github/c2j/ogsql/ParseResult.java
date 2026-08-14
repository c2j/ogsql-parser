package io.github.c2j.ogsql;

import com.fasterxml.jackson.databind.JsonNode;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Result of {@code op: parse} — mirrors the serve-stdio protocol's {@code result}
 * object. SQL syntax problems surface as non-empty {@link #errors()}, never as an
 * exception (see {@link OgsqlException} for what actually throws).
 */
public final class ParseResult {

    private final JsonNode statements;
    private final JsonNode errors;
    private final JsonNode queryFingerprints;
    private final JsonNode comments;

    static ParseResult from(JsonNode result) {
        return new ParseResult(
            result.path("statements"),
            result.path("errors"),
            result.path("query_fingerprints"),
            result.path("comments"));
    }

    private ParseResult(JsonNode statements, JsonNode errors, JsonNode queryFingerprints, JsonNode comments) {
        this.statements = statements;
        this.errors = errors;
        this.queryFingerprints = queryFingerprints;
        this.comments = comments;
    }

    /** Parsed statement ASTs (StatementInfo array, same shape as the HTTP/MCP API). */
    public JsonNode statements() {
        return statements;
    }

    /** Parser errors/warnings; empty for clean parses. */
    public JsonNode errors() {
        return errors;
    }

    /** Query fingerprints ({@code normalized_sql} + {@code fingerprint}). */
    public JsonNode queryFingerprints() {
        return queryFingerprints;
    }

    /** Preserved comments, or {@code null} when none were requested/present. */
    public JsonNode comments() {
        return comments.isNull() ? null : comments;
    }

    /** Compact JSON of the {@code result} object — feeds directly into {@link Ogsql#json2sql(String)}. */
    public String resultJson() {
        return "{\"statements\":" + statements.toString() + "}";
    }

    public int statementCount() {
        return statements.isArray() ? statements.size() : 0;
    }

    public int errorCount() {
        return errors.isArray() ? errors.size() : 0;
    }
}
