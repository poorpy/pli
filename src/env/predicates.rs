use super::{Atom, Error, Number, Sexp};

pub fn is_atom(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(_) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}

pub fn is_boolean(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Bool(_)) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}
pub fn is_char(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Char(_)) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}
pub fn is_integer(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Number(Number::Int(_))) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}

pub fn is_float(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Number(Number::Float(_))) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}

pub fn is_number(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Number(_)) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}

pub fn is_cons(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Cons { .. } = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}

pub fn is_symbol(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Symbol(_)) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}
pub fn is_function(sexp: &Sexp) -> Result<Sexp, Error> {
    // TODO: update after implementing lambdas
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Func { .. }) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}
pub fn is_null(sexp: &Sexp) -> Result<Sexp, Error> {
    let mut iter = sexp.into_iter();
    let arg = iter.next();
    let rest = iter.next();
    if arg.is_none() || rest.is_some() {
        return Err(Error::Reason("function takes 1 argument".to_owned()));
    }

    if let Sexp::Atom(Atom::Nil) = arg.expect("empty value after check") {
        return Ok(Sexp::Atom(Atom::Bool(true)));
    }

    Ok(Sexp::Atom(Atom::Bool(false)))
}
