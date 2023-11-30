
pub mod token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0' 
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        // skip whitespace
        match self.ch {
            '+' => {
                tok = token::Token::Plus(self.ch);
            },
            '0' => {
                tok = token::Token::Eof;
            },
            _ => todo!()
        }
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::lexer::token::Token;

    #[test]
    fn test_lexer1() {
        let input = "+-*/";
        let res = Lexer::new(input.chars().collect()).next_token();
        println!("{:?}", res);
        let expected_results = vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide
        ];
    }
}
