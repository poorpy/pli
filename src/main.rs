// use plib::env::Env;
// use plib::eval::eval;
use plib::lexer::tokenize;
use plib::parser::read_from_tokens;
use std::io;

fn main() {
    let samples = [
        "nil".to_owned(),
        "6".to_owned(),
        "6.5".to_owned(),
        "true".to_owned(),
        "symbol?".to_owned(),
        "'symbol?".to_owned(),
        "symbol?".to_owned(),
        "'(1)".to_owned(),
        "'1".to_owned(),
        "(bool? true)".to_owned(),
        "(number? true)".to_owned(),
        "(number? 1.0)".to_owned(),
        "(int? 1)".to_owned(),
        "(cons? '(1 2 3))".to_owned(),
        "(cons? (cons true false))".to_owned(),
        "(cons true false)".to_owned(),
        "'(1 2 3)".to_owned(),
    ];
    for sample in samples.iter() {
        simple_eval(sample.clone());
    }
}

fn simple_eval(code: String) {
    // let mut default_env = Env::default_env();
    let tokens = tokenize(code);
    if let Ok(mut deque) = tokens {
        let res = read_from_tokens(&mut deque).unwrap();
        println!("{}", res)
        // match eval(&res, &mut default_env) {
        //     Ok(sexp) => println!("{}", sexp),
        //     Err(err) => println!("{}", err),
        // }
    } else if let Err(err) = tokens {
        println! {"{}", err}
    }
}

#[allow(dead_code)]
fn slurp_exp() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}
