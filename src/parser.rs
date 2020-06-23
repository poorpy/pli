use std::collections::VecDeque;
use super::lexer::Token;
use super::sexp::{Sexp, Error, Atom};
use super::number::Number;

pub fn read_from_tokens(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
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
    let mut vec: Vec<Sexp> = Vec::new();
    while let Some(token) = tokens.front() {
        println!("{:?}", token);
        if *token == Token::RParen {
            tokens.pop_front();
            let start = Sexp::Cons {
                car: Box::new(vec.pop().unwrap()),
                cdr: Box::new(Sexp::Atom(Atom::Nil))
            };
            return Ok(vec.into_iter().rfold(start, |acc, x| Sexp::Cons { car: Box::new(x), 
                                                                    cdr: Box::new(acc)}));
        }

        if let Ok(sexp) = read_from_tokens(tokens) {
            vec.push(sexp);
        }
    }
    println!("{:?}", tokens);
    println!("{:?}", vec);
    Err(Error::Reason("nie pykło byqu".to_owned()))
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

fn parse_quoted(_tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    unimplemented!()
}
