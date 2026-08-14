package io.github.c2j.ogsql;

/**
 * Unchecked exception thrown by the ogsql Java connector for protocol-level failures
 * (child process died, timeout, unknown op, invalid input, etc.).
 *
 * <p>Note the contract: SQL syntax/semantics problems are <em>not</em> protocol errors —
 * {@link Ogsql#parse(String)} returns a {@link ParseResult} with non-empty errors and
 * {@link Ogsql#validate(String)} returns {@code valid == false}; no exception is thrown.</p>
 */
public class OgsqlException extends RuntimeException {

    private final String code;

    public OgsqlException(String message) {
        this("ERROR", message);
    }

    public OgsqlException(String code, String message) {
        super(message);
        this.code = code;
    }

    public OgsqlException(String message, Throwable cause) {
        super(message, cause);
        this.code = "ERROR";
    }

    /** Protocol error code, e.g. {@code PROTOCOL_ERROR}, {@code TOO_DEEP}, {@code UNKNOWN_OP}. */
    public String code() {
        return code;
    }
}
