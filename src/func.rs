use crate::data::{
    Symbol,
    SExpr,
};

macro_rules! atom {
    ($a:expr) => {
        SExpr::Atom($a)
    };
    () => {
        SExpr::Atom(Symbol::Nil)  
    };
}

// atom: return `T` if the arg is atom;`F` otherwise.
pub fn atom(a: SExpr) -> SExpr {
    match &a {
        SExpr::Atom(_) => atom!(Symbol::T),
        SExpr::List{car: _, cdr: _} => atom!(Symbol::F)
    }
}

// cons: given 2 SExpr, returns (left . right). 
pub fn cons(left: SExpr, right: SExpr) -> SExpr {
    SExpr::List{
        car: Box::new(left),
        cdr: Box::new(right)
    }
}

pub fn eq(a: SExpr, b: SExpr) -> SExpr {
    if a == b {
        atom!(Symbol::T)
    }
    else{
        atom!(Symbol::F)
    }
}

pub fn car(s: SExpr) -> SExpr {
    match s {
        SExpr::List{car: a, cdr: _} => *a,
        _ => atom!(Symbol::Nil)
    }
}

pub fn cdr(s: SExpr) -> SExpr {
    match s {
        SExpr::List{car: _, cdr: a} => *a,
        _ => atom!(Symbol::Nil)
    }
}
