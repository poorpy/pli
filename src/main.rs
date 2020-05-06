use plib::lexer::{tokenize, Token};
use std::boxed::Box;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut tokens = tokenize("\"hello world\"".to_owned()).unwrap();
    println!("{:?}", read_from_tokens(&mut tokens));
}

#[allow(dead_code)]
fn slurp_exp() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

use std::rc::Rc;
use std::string;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i32),
    Float(f64),
}

impl string::ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Int(i) => i.to_string(),
            Number::Float(f) => f.to_string(),
        }
    }
}

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

#[derive(Clone)]
pub struct Lambda {
    pub params_exp: Rc<Sexp>,
    pub body_exp: Rc<Sexp>,
}

#[derive(Debug)]
pub enum Error {
    Reason(String),
}

#[derive(Debug)]
pub enum Sexp {
    Atom(Atom),
    Cons { car: Box<Sexp>, cdr: Box<Sexp> },
}

fn read_from_tokens(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    let token = tokens.pop_front();
    match token {
        None => Err(Error::Reason("unexpected EOF".to_owned())),
        Some(lex) => match lex {
            Token::LParen => parse_list(tokens),
            Token::RParen => Err(Error::Reason("unexpected ')'".to_owned())),
            Token::Quote => parse_quoted(tokens),
            Token::Symbol(s) => Ok(parse_atom(&s)),
            Token::String(s) => Ok(parse_string(&s)),
        },
    }
}

fn parse_list(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    unimplemented!()
}

fn parse_string(token: &str) -> Sexp {
    let mut rev = token.chars().rev();

    if let Some(first) = rev.next() {
        let mut sexp = Box::new(Sexp::Cons {
            car: Box::new(Sexp::Atom(Atom::Char(first))),
            cdr: Box::new(Sexp::Atom(Atom::Nil)),
        });

        for c in rev {
            let new = Box::new(Sexp::Cons {
                car: Box::new(Sexp::Atom(Atom::Char(c))),
                cdr: sexp,
            });

            sexp = new;
        }

        return *sexp;
    }

    return Sexp::Atom(Atom::Nil);
}

fn parse_atom(token: &str) -> Sexp {
    match token.as_ref() {
        "true" => Sexp::Atom(Atom::Bool(true)),
        "false" => Sexp::Atom(Atom::Bool(false)),
        "nil" => Sexp::Atom(Atom::Nil),
        _ => match token.parse::<i32>() {
            Ok(i) => Sexp::Atom(Atom::Number(Number::Int(i))),
            Err(_) => match token.parse::<f64>() {
                Ok(f) => Sexp::Atom(Atom::Number(Number::Float(f))),
                Err(_) => Sexp::Atom(Atom::Symbol(token.to_owned())),
            },
        },
    }
}

fn parse_quoted(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    unimplemented!()
}
