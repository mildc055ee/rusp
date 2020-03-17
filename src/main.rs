mod data;
mod parser;
use parser::Parser;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();

    let mut p = Parser::new(&input.trim().to_string());
    p.lex().ok();

    for node in p.ast {
        println!("{}", node);
    }
}