use plib::{lexer, number, parser, sexp};

#[test]
fn parse_eof_error() {
    let mut vec = lexer::tokenize("".to_owned()).unwrap();
    let err = parser::read_from_tokens(&mut vec);
    let expected = "unexpected EOF".to_owned();
    let mut to_check = "".to_owned();
    if let Err(sexp::Error::Reason(r)) = err {
        to_check = r
    }
    assert_eq!(to_check, expected)
}

#[test]
fn parse_rparen_error() {
    let mut vec = lexer::tokenize(")".to_owned()).unwrap();
    let err = parser::read_from_tokens(&mut vec);
    let expected = "unexpected ')'".to_owned();
    let mut to_check = "".to_owned();
    if let Err(sexp::Error::Reason(r)) = err {
        to_check = r
    }
    assert_eq!(to_check, expected)
}

#[test]
fn parse_lparen_error() {
    let mut vec = lexer::tokenize("(".to_owned()).unwrap();
    let err = parser::read_from_tokens(&mut vec);
    let expected = "missing ')' in expression: (".to_owned();
    let mut to_check = "".to_owned();
    if let Err(sexp::Error::Reason(r)) = err {
        to_check = r
    }
    assert_eq!(to_check, expected)
}

#[test]
fn parse_empty_list() {
    let mut tokens = lexer::tokenize("()".to_owned()).unwrap();
    if let Ok(sexp::Sexp::List(vec)) = parser::read_from_tokens(&mut tokens) {
        assert_eq!(vec.len(), 0)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_nil() {
    let mut tokens = lexer::tokenize("nil".to_owned()).unwrap();
    if let Ok(sexp::Sexp::List(vec)) = parser::read_from_tokens(&mut tokens) {
        assert_eq!(vec.len(), 0)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_nested_list() {
    let mut tokens = lexer::tokenize("(())".to_owned()).unwrap();
    if let Ok(sexp::Sexp::List(outer)) = parser::read_from_tokens(&mut tokens) {
        if let sexp::Sexp::List(ref inner) = outer[0] {
            assert_eq!(inner.len(), 0)
        } else {
            assert!(false)
        }
    } else {
        assert!(false)
    }
}

#[test]
fn parse_true() {
    let mut tokens = lexer::tokenize("true".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::Bool(t))) = parser::read_from_tokens(&mut tokens) {
        assert!(t)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_false() {
    let mut tokens = lexer::tokenize("false".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::Bool(f))) = parser::read_from_tokens(&mut tokens) {
        assert!(!f)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_int() {
    let mut tokens = lexer::tokenize("2137".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::Number(number::Number::Int(i)))) =
        parser::read_from_tokens(&mut tokens)
    {
        assert_eq!(i, 2137)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_float() {
    let mut tokens = lexer::tokenize("21.37".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::Number(number::Number::Float(f)))) =
        parser::read_from_tokens(&mut tokens)
    {
        assert_eq!(f, 21.37)
    } else {
        assert!(false)
    }
}

#[test]
fn parse_symbol() {
    let mut tokens = lexer::tokenize("symbol".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::Symbol(s))) = parser::read_from_tokens(&mut tokens) {
        assert_eq!(s, "symbol")
    } else {
        assert!(false)
    }
}

#[test]
fn parse_string() {
    let mut tokens = lexer::tokenize("\"symbol\"".to_owned()).unwrap();
    if let Ok(sexp::Sexp::Atom(sexp::Atom::String(s))) = parser::read_from_tokens(&mut tokens) {
        assert_eq!(s, "symbol")
    } else {
        assert!(false)
    }
    
}
