use std::io;
mod lexer;
use lexer::tokenize;

fn main() {
    println!("{:?}", tokenize("(dudu \"Hello, world!\")".to_owned()));
}

#[allow(dead_code)]
fn slurp_exp() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

