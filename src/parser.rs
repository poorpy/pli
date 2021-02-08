use super::lexer::Token;
use super::number::Number;
use super::sexp::{Atom, Error, Sexp};
use std::collections::VecDeque;

pub fn read_from_tokens(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    let token = tokens.pop_front();
    match token {
        None => Err(Error::Reason("unexpected EOF".to_owned())),
        Some(lex) => match lex {
            Token::LParen => parse_list(tokens),
            Token::RParen => Err(Error::Reason("unexpected ')'".to_owned())),
            Token::Quote => parse_quoted(tokens),
            Token::Symbol(s) => Ok(parse_atom(&s)),
            Token::String(s) => Ok(Sexp::Atom(Atom::String(s)))
        },
    }
}

fn parse_list(tokens: &mut VecDeque<Token>) -> Result<Sexp, Error> {
    let mut vec: Vec<Sexp> = Vec::new();
    let mut terminated = false;
    while let Some(token) = tokens.front() {
        if *token == Token::RParen {
            tokens.pop_front();
            terminated = true;
            break;
        }

        let sexp = read_from_tokens(tokens)?;
        vec.push(sexp);
    }

    if !terminated {
        let mut msg = format!("missing ')' in expression: {}", Sexp::List(vec));
        msg.pop();
        return Err(Error::Reason(msg));
    }

    Ok(Sexp::List(vec))
}

fn parse_atom(token: &str) -> Sexp {
    match token.as_ref() {
        "true" => Sexp::Atom(Atom::Bool(true)),
        "false" => Sexp::Atom(Atom::Bool(false)),
        "nil" => Sexp::List(vec![]),
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
    let ret = vec![
        Sexp::Atom(Atom::Symbol("'".to_owned())),
        read_from_tokens(tokens)?
    ];
    Ok(Sexp::List(ret))
}
