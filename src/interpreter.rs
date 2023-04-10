use crate::parser::Expr;

pub fn interpret(expr: &Expr) {
    match expr {
        Expr::Log(expr) => {
            interpret(expr);
        }
        Expr::StringLiteral(s) => println!("{}", s),
        Expr::NumberLiteral(n) => println!("{}", n),
        _ => {}
    }
}