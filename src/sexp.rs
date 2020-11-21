use super::number::Number;
use std::fmt;
use std::iter::{IntoIterator, Iterator};
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

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Sexp::Atom(a) => write!(f, "{}", a),
            Sexp::Cons { car, cdr } => write!(f, "({} . {})", car, cdr),
        }
    }
}

impl<'a> IntoIterator for &'a Sexp {
    type Item = &'a Sexp;
    type IntoIter = SexpIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SexpIntoIterator { item: Some(self) }
    }
}

pub struct SexpIntoIterator<'a> {
    item: Option<&'a Sexp>,
}

// impl<'a> SexpIntoIterator<'a> {
//     pub fn take(&self, n: usize) -> Result<Sexp, >
// }

impl<'a> Iterator for SexpIntoIterator<'a> {
    type Item = &'a Sexp;

    fn next(&mut self) -> Option<&'a Sexp> {
        if let Some(i) = self.item {
            match i {
                a @ Sexp::Atom(_) => {
                    self.item = None;
                    return Some(a);
                }
                Sexp::Cons { car, cdr } => {
                    if let Sexp::Atom(Atom::Nil) = **cdr {
                        self.item = None;
                    } else {
                        self.item = Some(cdr);
                    }
                    return Some(car);
                }
            }
        }
        None
    }
}
