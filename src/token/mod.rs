pub mod encoding;
pub mod keyword;
pub mod tokenizer;

pub use encoding::decode_sql_file;
pub use keyword::Keyword;

/// Span represents a range of bytes in the source SQL text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// Human-readable source location for error reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceLocation {
    /// 1-based line number
    pub line: usize,
    /// 1-based column number
    pub column: usize,
    /// 0-based byte offset in source
    pub offset: usize,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }
}

/// A token with its location in the source text.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
    pub location: SourceLocation,
}

/// SQL token types for the openGauss lexer.
///
/// Based on the token declarations in gram.y and the lexical rules in scan.l.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    DollarString {
        tag: Option<String>,
        body: String,
    },
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
    /// Less-than-or-equal <=
    OpLe,
    /// Not-equal <> (ANSI)
    OpNe,
    /// Left shift <<
    OpShiftL,
    /// Greater-than-or-equal >=
    OpGe,
    /// Right shift >>
    OpShiftR,
    /// Not-equal != (non-ANSI)
    OpNe2,
    /// Double exclamation !! (factorial prefix)
    OpDblBang,
    /// Double-pipe || (string concatenation)
    OpConcat,
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

    /// Optimizer hint: /*+ ... */
    Hint(String),
}

impl Token {
    pub fn as_op_str(&self) -> Option<&str> {
        match self {
            Token::Op(s) => Some(s),
            Token::OpLe => Some("<="),
            Token::OpNe => Some("<>"),
            Token::OpShiftL => Some("<<"),
            Token::OpGe => Some(">="),
            Token::OpShiftR => Some(">>"),
            Token::OpNe2 => Some("!="),
            Token::OpDblBang => Some("!!"),
            Token::OpConcat => Some("||"),
            _ => None,
        }
    }
}
