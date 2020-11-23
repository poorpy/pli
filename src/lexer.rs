use std::collections::VecDeque;
use std::fmt;
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

#[derive(Debug, PartialEq)]
pub enum LexerError {
    Reason(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            LexerError::Reason(res) => write!(f, "Lexer error: {}", res),
        }
    }
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
    fn is_forbidden(c: char) -> bool {
        match c {
            '\"' | '\'' => true,
            _ => false,
        }
    }

    fn is_terminal(c: char) -> bool {
        match c {
            c if c.is_whitespace() => true,
            '(' | ')' => true,
            _ => false,
        }
    }

    let mut symbol = String::new();
    let mut forbidden: Vec<char> = Vec::new();
    while let Some(c) = code.peek() {
        if is_terminal(*c) {
            if forbidden.is_empty() {
                return Ok(Token::Symbol(symbol));
            }
            let msg = format!("invalid characters: {:?} in symbol: {}", forbidden, symbol);
            return Err(LexerError::Reason(msg));
        }

        let c = code.next().unwrap();
        if is_forbidden(c) {
            forbidden.push(c)
        }
        symbol.push(c)
    }

    if !forbidden.is_empty() {
        let msg = format!("invalid characters: {:?} in symbol: {}", forbidden, symbol);
        return Err(LexerError::Reason(msg));
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
