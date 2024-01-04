#[derive(PartialEq, Debug)]
pub enum Token {
    // Keywords
    If,
    Else,
    While,
    For,
    Struct,
    Break,
    Continue,
    Return,
    New,

    // Literals
    True,
    False,
    Identifier(String),
    IntLiteral(String),
    FloatLiteral(String),
    StringLiteral(String),
    NullLiteral,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Or,
    And,
    Not,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    Assignment,
    Increment,
    Decrement,
    Hash,
    LeftShift,
    RightShift,

    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Semicolon,

    // Special
    Illegal,
    Eof,
}
