package io.github.c2j.ogsql;

import com.fasterxml.jackson.databind.JsonNode;

/**
 * Result of {@code op: validate}. {@link #valid()} is {@code false} when the SQL
 * has hard parse errors, package-consistency issues, or undefined-variable issues;
 * warnings (reserved-keyword-as-identifier etc.) do not fail validation.
 */
public final class Validation {

    private final boolean valid;
    private final JsonNode statements;
    private final JsonNode errors;
    private final JsonNode packageErrors;
    private final JsonNode undefinedVariableErrors;

    static Validation from(JsonNode result) {
        return new Validation(
            result.path("valid").asBoolean(false),
            result.path("statements"),
            result.path("errors"),
            result.path("package_errors"),
            result.path("undefined_variable_errors"));
    }

    private Validation(boolean valid, JsonNode statements, JsonNode errors,
                       JsonNode packageErrors, JsonNode undefinedVariableErrors) {
        this.valid = valid;
        this.statements = statements;
        this.errors = errors;
        this.packageErrors = packageErrors;
        this.undefinedVariableErrors = undefinedVariableErrors;
    }

    public boolean valid() {
        return valid;
    }

    public JsonNode statements() {
        return statements;
    }

    public JsonNode errors() {
        return errors;
    }

    public JsonNode packageErrors() {
        return packageErrors;
    }

    public JsonNode undefinedVariableErrors() {
        return undefinedVariableErrors;
    }

    public int errorCount() {
        return errors.isArray() ? errors.size() : 0;
    }
}
