use std::collections::VecDeque;
use std::io;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    println!("{:?}", tokenize("(\"Hello, world!\")".to_owned()));
}

#[allow(dead_code)]
fn slurp_exp() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

#[derive(Debug)]
pub enum Token {
    Symbol(String),
    String(String),
    Quote,
    LParen,
    RParen,
}

#[derive(Debug)]
pub enum LexerError {
    Reason(String),
}

pub fn tokenize(code: String) -> VecDeque<Token> {
    let mut chars = code.chars().peekable();
    let mut tokens: VecDeque<Token> = VecDeque::new();
    while let Some(c) = chars.peek() {
        if c == &'('  {
            tokens.push_back(Token::LParen);
            chars.next();
            continue;
        } else if c == &')' {
            tokens.push_back(Token::RParen);
            chars.next();
            continue;
        } else if c == &'\"' {
            chars.next();
            tokens.push_back(tokenize_string(&mut chars).unwrap())
        }

    }

    tokens
}

#[allow(dead_code)]
fn tokenize_symbol(code: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut symbol = String::new();
    while let Some(c) = code.next() {
        if c.is_whitespace() {
            return Ok(Token::Symbol(symbol));
        }
        symbol.push(c)
    }
    Err(LexerError::Reason("dunno mate".to_owned()))
}

#[allow(dead_code)]
fn tokenize_string(code: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut string = String::new();
    while let Some(c) = code.next() {
        if c == '\"' {
            return Ok(Token::String(string));
        }
        string.push(c)
    }

    Err(LexerError::Reason("missing closing quote".to_owned()))
}

