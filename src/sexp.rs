use super::number::Number;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Atom {
    Nil,
    Char(char),
    Bool(bool),
    Symbol(String),
    Number(Number),
    Func {
        fun: fn(&Sexp) -> Result<Sexp, Error>,
        name: &'static str,
    },
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
            Atom::Func { name, .. } => write!(f, "builtin function {}", name),
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
        write!(f, "Error: {}", res)
    }
}

#[derive(Clone)]
pub enum Sexp {
    Atom(Atom),
    Cons { car: Box<Sexp>, cdr: Box<Sexp> },
}

impl Sexp {
    pub fn len(&self) -> usize {
        if let Sexp::Cons { cdr, .. } = self {
            let mut counter: usize = 1;
            let mut next: &Sexp = &(**cdr);
            loop {
                if let Sexp::Cons { cdr, .. } = next {
                    counter += 1;
                    next = &(**cdr);
                } else {
                    break;
                }
            }
            return counter;
        }
        // default return if Sexp is not Cons
        0
    }

    pub fn car(&self) -> &Sexp {
        match self {
            Sexp::Atom(_) => self,
            Sexp::Cons { car, .. } => car,
        }
    }
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Sexp::Atom(a) => write!(f, "{}", a),
            Sexp::Cons { car, cdr } => write!(f, "({} . {})", car, cdr),
        }
    }
}
