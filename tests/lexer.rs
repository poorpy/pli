use plib::lexer;
use std::collections::VecDeque;

#[test]
fn tokenize_parens() {
    let vec = vec![lexer::Token::LParen, lexer::Token::RParen];
    let tokens = lexer::tokenize("()".to_owned()).unwrap();
    let matching = vec
        .iter()
        .zip(tokens.iter())
        .filter(|&(a, b)| a == b)
        .count();
    assert_eq!(matching, tokens.len())
}

#[test]
fn tokenize_symbol() {
    let mut expected = VecDeque::new();
    expected.push_back(lexer::Token::Symbol("symbol?".to_owned()));
    let result = lexer::tokenize("symbol?".to_owned()).unwrap();
    assert_eq!(expected, result)
}

#[test]
fn tokenize_symbol_error() {
    let is_err = match lexer::tokenize("dd'dd".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(is_err);
}

#[test]
fn tokenize_string() {
    let mut expected = VecDeque::new();
    expected.push_back(lexer::Token::String("string".to_owned()));
    let result = lexer::tokenize("\"string\"".to_owned()).unwrap();
    assert_eq!(expected, result)
}

#[test]
fn tokenize_string_error() {
    let is_err = match lexer::tokenize("\"ddd".to_owned()) {
        Ok(_) => false,
        Err(_) => true,
    };
    assert!(is_err)
}

#[test]
fn tokenize() {
    let mut expected = VecDeque::new();
    expected.push_back(lexer::Token::LParen);
    expected.push_back(lexer::Token::Symbol("string?".to_owned()));
    expected.push_back(lexer::Token::String("string".to_owned()));
    expected.push_back(lexer::Token::RParen);
    let result = lexer::tokenize("(string? \"string\")".to_owned()).unwrap();
    assert_eq!(expected, result)
}
