pub mod keyword;
pub mod tokenizer;
pub mod encoding;

pub use keyword::Keyword;
pub use encoding::decode_sql_file;

/// Span represents a range of bytes in the source SQL text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A token with its location in the source text.
#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

/// SQL token types for the openGauss lexer.
///
/// Based on the token declarations in gram.y and the lexical rules in scan.l.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// End of input
    Eof,

    // --- Literals ---
    /// Integer constant
    Integer(i64),
    /// Floating point constant (stored as string to preserve precision)
    Float(String),
    /// Single-quoted string literal
    StringLiteral(String),
    /// Bit string literal (b'...' or B'...')
    BitString(String),
    /// Hex string literal (x'...' or X'...')
    HexString(String),
    /// Dollar-quoted string literal ($$...$$ or $tag$...$tag$)
    DollarString(String),
    /// National character string (N'...')
    NationalString(String),
    /// Escape string literal (E'...')
    EscapeString(String),

    // --- Identifiers ---
    /// Unquoted identifier (may also be a keyword if lookup matches)
    Ident(String),
    /// Double-quoted identifier
    QuotedIdent(String),
    /// SQL keyword
    Keyword(Keyword),

    // --- Parameters ---
    /// Positional parameter ($1, $2, ...)
    Param(i32),

    // --- Operators ---
    /// Multi-character operator (>=, !=, <>, ||, etc.)
    Op(String),
    /// Typecast operator ::
    Typecast,
    /// Range operator ..
    DotDot,
    /// Assignment operator :=
    ColonEquals,
    /// Parameter assignment operator =>
    ParamEquals,
    /// Oracle-style outer join operator (+)
    PlusJoin,

    // --- Punctuation ---
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Lt,
    Gt,
    Eq,
    At,

    // --- Special ---
    /// @@ variable (session/global parameter)
    SetIdent(String),
}
