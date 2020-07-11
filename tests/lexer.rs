use plib::lexer;

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
