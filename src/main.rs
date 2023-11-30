mod lexer;

fn main() {
    let input = String::from("let a = 5;");
    let mut l = lexer::Lexer::new(input.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::Eof {
            break
        } else {
            println!("{:?}", token);
        }
    }
}
