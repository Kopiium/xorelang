use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    Log(Box<Expr>),
    StringLiteral(String),
    NumberLiteral(f64),
}

pub fn parse(tokens: &[Token]) -> Expr {
    let mut iter = tokens.iter().peekable();

    match iter.next() {
        Some(Token::Log) => {
            iter.next(); // skip colon
            let expr = parse(iter.cloned().collect::<Vec<Token>>().as_slice());
            Expr::Log(Box::new(expr))
        }
        Some(Token::StringLiteral(s)) => Expr::StringLiteral(s.to_string()),
        Some(Token::NumberLiteral(n)) => Expr::NumberLiteral(*n),
        _ => panic!("Unexpected token"),
    }
}