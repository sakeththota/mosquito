
pub mod token;
use token::Token;
use anyhow::Result;
pub struct Lexer {
    input: Vec<u8>,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: b'0'
        };
        lex.read_char();
        return lex;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input[self.read_position];
        }
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read_char();
        }
        if self.peek() == b'l' || self.peek() == b'L' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }
    
    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();
        let tok: Token = match self.ch {
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'%' => Token::Modulo,
            b'!' => Token::Not,
            b'<' => Token::LessThan,
            b'>' => Token::GreaterThan,
            b'=' => Token::Assignment,
            b'#' => Token::Hash,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'[' => Token::LBracket,
            b']' => Token::RBracket,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b',' => Token::Comma,
            b'.' => Token::Dot,
            b';' => Token::Semicolon,
            b'"' => todo!(),
            0 => Token::Eof,
            b'0'..=b'9' => {
                let num: String = self.read_number();
                if num.contains('.') {
                    return Ok(Token::FloatLiteral(num));
                } else { 
                    return Ok(Token::IntLiteral(num));
                }
            },
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_identifier();
                return Ok(match identifier.as_str() {
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "struct" => Token::Struct,
                    "break" => Token::Break,
                    "continue" => Token::Continue,
                    "return" => Token::Return,
                    "new" => Token::New,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Identifier(identifier),
                });
            },
            _ => Token::Illegal
        };
        println!("{:?}", tok);
        self.read_char();
        return Ok(tok);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operators() -> Result<()> {
        let input: &str = "+-*/%!<>=#";
        let mut lexer: Lexer = Lexer::new(input.into());
        let tokens: [Token; 10] = [
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Modulo,
            Token::Not,
            Token::LessThan,
            Token::GreaterThan,
            Token::Assignment,
            Token::Hash
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn test_ints_longs_arithmetic() -> Result<()> {
        let input = "5 + 10 - 15 * 20 / 25 % 30";
        let mut lexer = Lexer::new(input.into());

        let tokens = [
            Token::IntLiteral(String::from("5")),
            Token::Plus,
            Token::IntLiteral(String::from("10")),
            Token::Minus,
            Token::IntLiteral(String::from("15")),
            Token::Multiply,
            Token::IntLiteral(String::from("20")),
            Token::Divide,
            Token::IntLiteral(String::from("25")),
            Token::Modulo,
            Token::IntLiteral(String::from("30")),
            Token::Eof
        ];
        
        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
