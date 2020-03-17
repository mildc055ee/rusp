use crate::data::SExpr;

pub struct Parser {
    src: String,
    pub ast: Vec<SExpr>,
}

impl Parser {
    pub fn new(s: &String) -> Parser {
        Parser {
            src: s.clone(),
            ast: Vec::<SExpr>::new(),
        }
    }

    pub fn lex(&mut self) -> Result<(),()> {
        let mut iter = self.src.chars();
        while let Some(c) = iter.next() {
            match c {
                '(' => self.ast.push(SExpr::Symbol(String::from("("))),
                ')' => self.ast.push(SExpr::Symbol(String::from(")"))),
                i if c.is_numeric() => {
                    let mut num = String::new();
                    num.push(i);
                    while let Some(n) = iter.next() {
                        if n.is_numeric() {
                            num.push(n);
                        }
                        else {
                            break;
                        }
                    }
                    self.ast.push(SExpr::Int(num.parse::<i32>().unwrap()));
                },
                _w if c.is_whitespace() => continue,
                _ => self.ast.push(SExpr::Symbol(c.to_string())),
            }
        }

        Ok(())
    }
}