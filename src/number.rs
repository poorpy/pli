use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Number {
    Int(i32),
    Float(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Number::Int(i) => write!(f, "{}", i),
            Number::Float(k) => write!(f, "{}", k)
        }
    }
}
