use super::{Error, Sexp};

pub fn clone_exp(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    Ok(arg.expect("missing argument after check").clone())
}

pub fn cons(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg1 = iter.next();
    let arg2 = iter.next();
    let rest = iter.next();
    if arg1.is_none() || arg2.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 2 arguments".to_owned()));
    }

    Ok(Sexp::Cons {
        car: Box::new(arg1.expect("missing argument after check").clone()),
        cdr: Box::new(arg2.expect("missing argument after check").clone()),
    })
}
