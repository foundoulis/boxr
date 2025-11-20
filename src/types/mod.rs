pub mod function;
pub mod scope;

use std::{
    fmt::{Display, Error, Formatter},
    sync::Arc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Cons {
    Value(ConsValue),
    Cell(Arc<Cons>, Arc<Cons>),
    Quoted(Arc<Cons>),
}

impl Cons {
    pub fn dequote(&self) -> Cons {
        match self {
            Cons::Quoted(q) => q.as_ref().clone(),
            _ => self.clone(),
        }
    }
    pub fn is_nil(&self) -> bool {
        match self {
            Cons::Value(ConsValue::NIL) => true,
            _ => false,
        }
    }
    pub fn is_quoted(&self) -> bool {
        match self {
            Cons::Quoted(_) => true,
            _ => false,
        }
    }
    pub fn car(&self) -> Cons {
        match self {
            Cons::Cell(car, _) => car.as_ref().clone(),
            _ => Cons::Value(ConsValue::NIL),
        }
    }
    pub fn cdr(&self) -> Cons {
        match self {
            Cons::Cell(_, cdr) => cdr.as_ref().clone(),
            _ => Cons::Value(ConsValue::NIL),
        }
    }
    pub fn split(&self) -> Option<(Cons, Cons)> {
        match self {
            Cons::Cell(car, cdr) => Some((car.as_ref().clone(), cdr.as_ref().clone())),
            _ => None,
        }
    }
}

impl FromIterator<Cons> for Cons {
    fn from_iter<T: IntoIterator<Item = Cons>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(v) => Cons::Cell(Arc::new(v), Arc::new(Cons::from_iter(iter))),
            None => Cons::Value(ConsValue::NIL),
        }
    }
}

#[derive(Debug)]
pub struct ConsIter(Arc<Cons>);

impl Iterator for ConsIter {
    type Item = Cons;

    fn next(&mut self) -> Option<Self::Item> {
        match Cons::clone(&self.0) {
            Cons::Value(ConsValue::NIL) => None,
            Cons::Cell(car, cdr) => {
                self.0 = cdr.clone();
                Some(Cons::clone(&car))
            }
            _ => None,
        }
    }
}

impl IntoIterator for Cons {
    type Item = Cons;
    type IntoIter = ConsIter;

    fn into_iter(self) -> Self::IntoIter {
        ConsIter(Arc::new(self))
    }
}

impl Display for Cons {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Cons::Value(v) => write!(f, "{}", v),
            Cons::Cell(car, cdr) => {
                write!(f, "({}", car)?;
                let rest_of_list = Cons::clone(cdr);
                for elem in rest_of_list {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            Cons::Quoted(q) => write!(f, "'{}", q),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConsValue {
    NIL,
    Symbol(String),
    String(String),
    Boolean(bool),
    Int(i64),
    Float(f64),
    Comment(String),
}

impl ConsValue {
    pub fn is_nil(&self) -> bool {
        match self {
            ConsValue::NIL => true,
            _ => false,
        }
    }
}

impl Display for ConsValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ConsValue::NIL => write!(f, "'()"),
            ConsValue::Symbol(s) => write!(f, "{}", s),
            ConsValue::String(s) => write!(f, "\"{}\"", s),
            ConsValue::Boolean(b) => write!(f, "{}", b),
            ConsValue::Int(i) => write!(f, "{}", i),
            ConsValue::Float(fl) => write!(f, "{}", fl),
            ConsValue::Comment(s) => write!(f, "; {}\n", s),
        }
    }
}
