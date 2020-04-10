pub mod data;
pub mod func;
pub mod lexer;
//mod env;
use lexer::Lexer;

fn main() {
    let src = "(+ (10 20))";
    let mut l = Lexer::new(src);
    println!("{:?}", l.lex().unwrap());
}
