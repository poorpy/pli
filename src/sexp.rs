use super::number::Number;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Atom {
    Nil,
    Char(char),
    Bool(bool),
    Symbol(String),
    Number(Number),
    // Func(fn(&Sexp) -> Result<Sexp, Error>),
    // Lambda(Lambda)
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Atom::Nil => write!(f, "nil"),
            Atom::Char(c) => write!(f, "{}", c),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub params_exp: Rc<Sexp>,
    pub body_exp: Rc<Sexp>,
}

#[derive(Debug)]
pub enum Error {
    Reason(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Error::Reason(res) = &*self;
        write!(f, "Parser error: {}", res)
    }
}

#[derive(Debug)]
pub enum Sexp {
    Atom(Atom),
    Cons { car: Box<Sexp>, cdr: Box<Sexp> },
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Sexp::Atom(a) => write!(f, "{}", a),
            Sexp::Cons { car, cdr } => write!(f, "({} . {})", car, cdr),
        }
    }
}
