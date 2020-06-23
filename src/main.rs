use plib::lexer::tokenize;
use plib::parser::read_from_tokens;
use std::io;

fn main() {
    let tokens = tokenize("(+ 1 2)".to_owned());
    if let Ok(mut deque) = tokens {
        let res = read_from_tokens(&mut deque);
        println!("{:?}", &res);
        match res {
            Ok(sexp) => println!("{}", sexp),
            Err(err) => println!("{:?}", err)
        }
    } else if let Err(err) = tokens{
        println!{"{}", err}
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



