use std::{
    fmt::{Display, Error, Formatter},
    sync::Arc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    List(Vec<Arc<Expr>>),
    QuotedList(Vec<Arc<Expr>>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Expr::Value(v) => write!(f, "{}", v),
            Expr::List(l) => {
                write!(f, "(")?;
                for (i, e) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            Expr::QuotedList(l) => {
                write!(f, "'(")?;
                for (i, e) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    NIL,
    Symbol(String),
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Comment(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Value::NIL => write!(f, "'()"),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Comment(s) => write!(f, ";{}", s),
        }
    }
}
