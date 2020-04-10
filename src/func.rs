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


#[test]
fn for_atom(){
    let a = atom!(Symbol::Nil);
    let b = SExpr::List{car: Box::new(atom!(Symbol::T)), cdr: Box::new(atom!(Symbol::Nil))};

    assert_eq!(atom!(Symbol::T), atom(a));
    assert_eq!(atom!(Symbol::F), atom(b));
}

#[test]
fn for_cons(){
    let a = atom!(Symbol::T);
    let b = atom!(Symbol::T);
    let expect = SExpr::List{car: Box::new(atom!(Symbol::T)), cdr: Box::new(atom!(Symbol::T))};

    assert_eq!(expect, cons(a, b));
}

#[test]
fn for_eq(){
    let a1 = atom!(Symbol::Int(10));
    let a2 = atom!(Symbol::Int(10));
    assert_eq!(atom!(Symbol::T), eq(a1, a2));

    let l1 = cons(atom!(Symbol::Nil), atom!(Symbol::T));
    let l2 = cons(atom!(Symbol::Nil), atom!(Symbol::T));
    assert_eq!(atom!(Symbol::T), eq(l1, l2));
}

#[test]
fn for_car(){
    let l1 = cons(atom!(Symbol::Int(32)), atom!(Symbol::Int(50)));
    assert_eq!(atom!(Symbol::Int(32)), car(l1));
}

#[test]
fn for_cdr(){
    let l1 = cons(atom!(Symbol::Int(32)), atom!(Symbol::Int(50)));
    assert_eq!(atom!(Symbol::Int(50)), cdr(l1));
}

