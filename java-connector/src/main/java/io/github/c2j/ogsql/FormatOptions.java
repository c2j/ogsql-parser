package io.github.c2j.ogsql;

/**
 * Formatting options for {@link Ogsql#format(String, FormatOptions)} — parameter
 * names and defaults match the serve-stdio protocol and the CLI.
 */
public final class FormatOptions {

    public static final FormatOptions DEFAULT = builder().build();

    private final int indent;
    private final String keywordCase; // preserve | upper | lower
    private final String commaStyle;  // trailing | leading
    private final int lineWidth;
    private final boolean uppercase;
    private final boolean mybatis;
    private final boolean noSelectNewline;
    private final boolean noLogicalNewline;
    private final boolean noSemicolonNewline;

    private FormatOptions(Builder b) {
        this.indent = b.indent;
        this.keywordCase = b.keywordCase;
        this.commaStyle = b.commaStyle;
        this.lineWidth = b.lineWidth;
        this.uppercase = b.uppercase;
        this.mybatis = b.mybatis;
        this.noSelectNewline = b.noSelectNewline;
        this.noLogicalNewline = b.noLogicalNewline;
        this.noSemicolonNewline = b.noSemicolonNewline;
    }

    public int indent() { return indent; }
    public String keywordCase() { return keywordCase; }
    public String commaStyle() { return commaStyle; }
    public int lineWidth() { return lineWidth; }
    public boolean uppercase() { return uppercase; }
    public boolean mybatis() { return mybatis; }
    public boolean noSelectNewline() { return noSelectNewline; }
    public boolean noLogicalNewline() { return noLogicalNewline; }
    public boolean noSemicolonNewline() { return noSemicolonNewline; }

    public static Builder builder() {
        return new Builder();
    }

    public static final class Builder {
        private int indent = 2;
        private String keywordCase = "preserve";
        private String commaStyle = "trailing";
        private int lineWidth = 120;
        private boolean uppercase;
        private boolean mybatis;
        private boolean noSelectNewline;
        private boolean noLogicalNewline;
        private boolean noSemicolonNewline;

        public Builder indent(int indent) { this.indent = indent; return this; }
        public Builder keywordCase(String keywordCase) { this.keywordCase = keywordCase; return this; }
        public Builder commaStyle(String commaStyle) { this.commaStyle = commaStyle; return this; }
        public Builder lineWidth(int lineWidth) { this.lineWidth = lineWidth; return this; }
        public Builder uppercase(boolean uppercase) { this.uppercase = uppercase; return this; }
        public Builder mybatis(boolean mybatis) { this.mybatis = mybatis; return this; }
        public Builder noSelectNewline(boolean noSelectNewline) { this.noSelectNewline = noSelectNewline; return this; }
        public Builder noLogicalNewline(boolean noLogicalNewline) { this.noLogicalNewline = noLogicalNewline; return this; }
        public Builder noSemicolonNewline(boolean noSemicolonNewline) { this.noSemicolonNewline = noSemicolonNewline; return this; }

        public FormatOptions build() {
            return new FormatOptions(this);
        }
    }
}
