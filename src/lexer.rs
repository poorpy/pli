use std::collections::VecDeque;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
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

/// Divides supplied string into collection of tokens
pub fn tokenize(code: String) -> Result<VecDeque<Token>, LexerError> {
    let mut chars = code.chars().peekable();
    let mut tokens: VecDeque<Token> = VecDeque::new();
    while let Some(c) = chars.peek() {
        if c == &'(' {
            tokens.push_back(Token::LParen);
            chars.next();
            continue;
        } else if c == &')' {
            tokens.push_back(Token::RParen);
            chars.next();
            continue;
        } else if c == &'\'' {
            tokens.push_back(Token::Quote);
            chars.next();
            continue;
        } else if c == &'\"' {
            chars.next();
            let token = tokenize_string(&mut chars)?;  
            tokens.push_back(token);
            continue;

        } else if (&c).is_whitespace() {
            chars.next();
            continue;
        }
        
        let token = tokenize_symbol(&mut chars)?;
        tokens.push_back(token);
    }

    Ok(tokens)
}

fn tokenize_symbol(code: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    let mut symbol = String::new();
    while let Some(c) = code.next() {
        if c.is_whitespace() {
            return Ok(Token::Symbol(symbol));
        }
        symbol.push(c)
    }

    Ok(Token::Symbol(symbol))
}

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
