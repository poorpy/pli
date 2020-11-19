use super::number::Number;
use super::sexp::{Atom, Error, Sexp};
use std::collections::HashMap;

macro_rules! add_function_to_env {
    ($ name : expr ; $ func : expr ; $ env : expr) => {
        $env.insert(
            $name.to_owned(),
            Sexp::Atom(Atom::Func {
                fun: $func,
                name: $name,
            }),
        )
    };
}

macro_rules! check_arity {
    ($ arg : expr , $ num : expr) => {
        if $arg.len() > $num {
            let msg = format!("wrong arity, function takes {} arguments", $num);
            return Err(Error::Reason(msg));
        };
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
        add_function_to_env!(
            "atom?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(_) = sexp.car() {
                    return Ok(Sexp::Atom(Atom::Bool(true)));
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "boolean?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Bool(_) = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "char?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Char(_) = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "integer?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Number(n) = a {
                        if let Number::Int(_) = n {
                            return Ok(Sexp::Atom(Atom::Bool(true)));
                        }
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "number?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Number(_) = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "cons?";
            |sexp| {
                if let Sexp::Cons { .. } = sexp {
                    return Ok(Sexp::Atom(Atom::Bool(true)));
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "symbol?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Symbol(_) = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "function?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp.car() {
                    if let Atom::Func {..} = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        add_function_to_env!(
            "null?";
            |sexp| {
                check_arity!(sexp, 1);
                if let Sexp::Atom(a) = sexp {
                    if let Atom::Nil = a {
                        return Ok(Sexp::Atom(Atom::Bool(true)));
                    }
                }
                Ok(Sexp::Atom(Atom::Bool(false)))
            };
            default
        );

        default
    }
}
