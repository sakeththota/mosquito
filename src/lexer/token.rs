#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    // Keywords
    IF,
    ELSE,
    WHILE,
    FOR,
    STRUCT,
    BREAK,
    CONTINUE,
    RETURN,
    NEW,
    // Identifiers
    Ident(Vec<char>),
    // Literal
    Int(Vec<char>),
    Float(Vec<char>),
    String(Vec<char>),
    Boolean(Vec<char>),
    Null(Vec<char>),
    // Operators
    Plus(char),
    Minus(char),
    Multiply(char),
    Divide(char),
    Modulo(char),
    Or(char),
    And(char),
    Not(char),
    LessThan(char),
    GreaterThan(char),
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal(char),
    NotEqual,
    Assignment(char),
    Increment,
    Decrement,
    Hash(char),
    LeftShift,
    RightShift,
    // Delimeter
    LParen(char),
    RParen(char),
    LBracket(char),
    RBracket(char),
    LBrace(char),
    RBrace(char),
    Comma(char),
    Dot(char),
    Semicolon(char),
    // Special Tokens
    Illegal,
    Eof,
}

