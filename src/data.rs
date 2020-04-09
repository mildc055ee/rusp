use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Str(String),
    Int(i64),
    T,
    F,
    Nil
}


#[derive(Debug, PartialEq, Eq)]
pub enum SExpr {
    Atom(Symbol),
    List{
        car: Box<SExpr>,
        cdr: Box<SExpr>
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Symbol::Str(s) => write!(f, "{}", s),
            Symbol::Int(i) => write!(f, "{}", i),
            Symbol::T => write!(f, "T"),
            Symbol::F => write!(f, "F"),
            Symbol::Nil => write!(f, "Nil"),
        }
    }
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            SExpr::Atom(a) => write!(f, "{}", a),
            SExpr::List{car: ref a, cdr: ref b} => write!(f, "({} . {})", &a, &b)
        }
    }
}

