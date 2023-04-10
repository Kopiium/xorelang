use std::env;
use std::fs;

mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.xor>", args[0]);
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read file");
    let tokens = lexer::tokenize(&input);
    let ast = parser::parse(&tokens);
    interpreter::interpret(&ast);
}