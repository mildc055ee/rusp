use std::fmt;

pub enum SExpr {
    Int(i32),
    Symbol(String),
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SExpr::Int(n) => write!(f, "Int({})", n),
            SExpr::Symbol(s) => write!(f, "Symbol({})", s),
        }
    }
}

