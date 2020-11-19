use super::env::Env;
use super::sexp::{Atom, Error, Sexp};

pub fn eval(exp: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    match exp {
        Sexp::Atom(a) => eval_atom(a, env),
        Sexp::Cons { car, cdr } => eval_cons(car, cdr, env),
    }
}

fn eval_atom(atom: &Atom, env: &mut Env) -> Result<Sexp, Error> {
    match atom {
        Atom::Nil => Ok(Sexp::Atom(Atom::Nil)),
        c @ Atom::Char(_) => Ok(Sexp::Atom(c.clone())),
        b @ Atom::Bool(_) => Ok(Sexp::Atom(b.clone())),
        Atom::Symbol(s) => env.search(&s),
        n @ Atom::Number(_) => Ok(Sexp::Atom(n.clone())),
        f @ Atom::Func { .. } => Ok(Sexp::Atom(f.clone())),
    }
}

fn eval_cons(car: &Sexp, cdr: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    match car {
        Sexp::Atom(a) => {
            if let Atom::Symbol(s) = a {
                if s == "'" {
                    return Ok(cdr.clone());
                }
            }
            let args = eval_args(cdr.car(), env)?;
            apply(a, &args, env)
        }
        cons @ Sexp::Cons { .. } => {
            let func = eval(cons, env)?;
            eval_cons(&func, cdr, env)
        }
    }
}

fn eval_args(args: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    match args {
        Sexp::Atom(a) => match a {
            Atom::Symbol(s) => env.search(s),
            _ => Ok(Sexp::Atom(a.clone())),
        },
        Sexp::Cons { car, cdr } => eval_cons(car, cdr, env),
    }
}

fn apply(func: &Atom, args: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    if let Atom::Func { fun, .. } = func {
        return fun(args);
    }

    if let Atom::Symbol(s) = func {
        if let Sexp::Atom(a) = env.search(s)? {
            return apply(&a, args, env);
        }
    }
    Err(Error::Reason(
        "first argument must be a function".to_owned(),
    ))
}
