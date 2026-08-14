package io.github.c2j.ogsql;

/**
 * One token from {@code op: tokenize}.
 */
public final class TokenInfo {

    private final String type;
    private final String value;
    private final int line;
    private final int column;

    TokenInfo(String type, String value, int line, int column) {
        this.type = type;
        this.value = value;
        this.line = line;
        this.column = column;
    }

    public String type() {
        return type;
    }

    public String value() {
        return value;
    }

    public int line() {
        return line;
    }

    public int column() {
        return column;
    }

    @Override
    public String toString() {
        return type + "(" + value + ") @" + line + ":" + column;
    }
}
