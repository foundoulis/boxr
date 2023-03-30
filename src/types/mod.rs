pub mod builtin;
pub mod function;
pub mod macros;
pub mod scope;
pub mod userfunctions;

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

impl FromIterator<Arc<Expr>> for Expr {
    fn from_iter<T: IntoIterator<Item = Arc<Expr>>>(iter: T) -> Self {
        Expr::List(iter.into_iter().collect())
    }
}

impl IntoIterator for Expr {
    type Item = Expr;
    type IntoIter = std::vec::IntoIter<Expr>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Expr::List(l) => l
                .into_iter()
                .map(|e| Expr::clone(&e))
                .collect::<Vec<Expr>>()
                .into_iter(),
            _ => panic!("Cannot convert non-list expression into iterator."),
        }
    }
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
    Quoted(Arc<Value>),
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
            Value::Comment(s) => write!(f, "; {}", s),
            Value::Quoted(q) => write!(f, "'{}", q),
        }
    }
}
