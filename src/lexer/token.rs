#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // literals
    Identifier,
    Integer,
    Float,
    String,

    // keywords
    Fn,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Return,
    True,
    False,
    Null,
    Not,
    And,
    Or,
    Pass,
    Import,
    From,
    Class,

    // operators
    Plus,
    Minus,
    Star,
    DoubleStar,
    Slash,
    DoubleSlash,
    Percent,
    Equal,
    DoubleEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    ColonEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    // symbols
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Dot,
    Arrow,
    Semicolon,

    // layout
    Newline,
    Indent,
    Dedent,

    // end
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub line: usize,
    pub col: usize,
    pub span: (usize, usize), // char offsets [start, end)
}
