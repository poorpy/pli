use super::number::Number;
use super::sexp::{Atom, Error, Sexp};
use std::collections::HashMap;
mod predicates;
use predicates::{
    is_atom, is_boolean, is_char, is_cons, is_float, is_function, is_integer, is_null, is_number,
    is_symbol,
};
mod special_forms;
use special_forms::{clone_exp, cons};

macro_rules! add_func_to_env {
    ($ name : expr, $ func : expr, $ env : expr) => {
        $env.insert(
            $name.to_owned(),
            Sexp::Atom(Atom::Func {
                fun: $func,
                name: $name,
            }),
        )
    };
}

pub struct Env<'a> {
    pub data: HashMap<String, Sexp>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env<'a>>) -> Env<'a> {
        Env {
            data: HashMap::new(),
            outer,
        }
    }
    pub fn search(&self, symbol: &String) -> Result<Sexp, Error> {
        if let Some(sexp) = self.data.get(symbol) {
            return Ok((*sexp).clone());
        } else if let Some(outer) = self.outer {
            return outer.search(symbol);
        }

        Err(Error::Reason(String::from(
            "Symbol not found in environment.",
        )))
    }

    pub fn insert(&mut self, symbol: String, sexp: Sexp) {
        self.data.insert(symbol, sexp);
    }

    pub fn default_env() -> Env<'a> {
        let mut default = Env::new(None);
        add_func_to_env!("'", clone_exp, default);
        add_func_to_env!("atom?", is_atom, default);
        add_func_to_env!("bool?", is_boolean, default);
        add_func_to_env!("char?", is_char, default);
        add_func_to_env!("cons?", is_cons, default);
        add_func_to_env!("float?", is_float, default);
        add_func_to_env!("function?", is_function, default);
        add_func_to_env!("int?", is_integer, default);
        add_func_to_env!("null?", is_null, default);
        add_func_to_env!("number?", is_number, default);
        add_func_to_env!("symbol?", is_symbol, default);
        add_func_to_env!("cons", cons, default);

        default
    }
}
