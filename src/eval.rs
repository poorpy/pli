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
        Sexp::Atom(a) => apply(a, cdr, env),
        cons @ Sexp::Cons { .. } => {
            let func = eval(cons, env)?;
            eval_cons(&func, cdr, env)
        }
    }
}

fn map_eval(sexp: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    let mut vec: Vec<Sexp> = Vec::new();
    for item in sexp.into_iter() {
        let res = eval(item, env)?;
        vec.push(res)
    }

    let mut rev = vec.iter().rev();
    if let Some(first) = rev.next() {
        let mut sexp = Box::new(Sexp::Cons {
            car: Box::new(first.clone()),
            cdr: Box::new(Sexp::Atom(Atom::Nil)),
        });

        for item in rev {
            let new = Box::new(Sexp::Cons {
                car: Box::new(item.clone()),
                cdr: sexp,
            });

            sexp = new;
        }

        return Ok(*sexp);
    }

    Err(Error::Reason("missing argument list".to_owned()))
}

fn apply(func: &Atom, args: &Sexp, env: &mut Env) -> Result<Sexp, Error> {
    if let Atom::Func { fun, name } = func {
        if name == &"'" {
            return fun(args);
        }
        let args = map_eval(args, env)?;
        return fun(&args);
    }

    if let Atom::Symbol(s) = func {
        let fun = env.search(s)?;
        return eval_cons(&fun, args, env);
    }

    Err(Error::Reason(
        "first argument must be a function".to_owned(),
    ))
}
