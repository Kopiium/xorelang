#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Log,
    Colon,
    StringLiteral(String),
    NumberLiteral(f64),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'l' if chars.peek() == Some(&'o') && chars.nth(1) == Some('g') => {
                tokens.push(Token::Log);
            }
            ':' => tokens.push(Token::Colon),
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                tokens.push(Token::StringLiteral(string));
            }
            c if c.is_digit(10) => {
                let mut number = c.to_string();
                while let Some(c) = chars.peek() {
                    if !c.is_digit(10) && *c != '.' {
                        break;
                    }
                    number.push(chars.next().unwrap());
                }
                tokens.push(Token::NumberLiteral(number.parse().unwrap()));
            }
            _ => {}
        }
    }

    tokens
}
