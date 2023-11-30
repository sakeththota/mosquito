#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    // Operators
    Add(char),
    Subtract(char),
    Multiply(char),
    Divide(char),
    Modulo(char),
    Or(char),
    And(char),
    Not(char),
    LessThan(char),
    GreaterThan(char),
    // LessThanOrEqual(char),
    // GreaterThanOrEqual(char),
    Equal(char),
    // NotEqual(char),
    Assignment(char),
    // Increment(char),
    // Decrement(char),
    Hash(char),
    // LeftShift(char),
    // RightShift(char),

    // Delimeters
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

