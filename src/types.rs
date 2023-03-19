use std::fmt::{Display, Debug};

#[derive(PartialEq, Eq, Clone)]
pub enum BoxrType<'a> {
    NIL,
    INT(i64),
    STR(&'a str),
    CELL(Box<BoxrType<'a>>, Box<BoxrType<'a>>),
}

impl<'a> BoxrType<'a> {
    /// Nil type
    pub fn nil() -> BoxrType<'a> {
        BoxrType::NIL
    }

    /// Cell function
    pub fn cons_cell(car: BoxrType<'a>, cdr: BoxrType<'a>) -> BoxrType<'a> {
        BoxrType::CELL(Box::new(car), Box::new(cdr))
    }
    pub fn car(self) -> Option<BoxrType<'a>> {
        match self {
            BoxrType::CELL(car, _) => Some(*car),
            _ => None,
        }
    }
    pub fn cdr(self) -> Option<BoxrType<'a>> {
        match self {
            BoxrType::CELL(_, cdr) => Some(*cdr),
            _ => None,
        }
    }
    pub fn decompose(self) -> Option<(BoxrType<'a>, BoxrType<'a>)> {
        if let BoxrType::CELL(car, cdr) = self {
            Some((*car, *cdr))
        } else {
            None
        }
    }
    pub fn top(&mut self) -> Option<(BoxrType<'a>, BoxrType<'a>)> {
        match self {
            BoxrType::CELL(car, cdr) => Some((*car.clone(), *cdr.clone())),
            _ => None
        }
    }
}

impl<'a> FromIterator<BoxrType<'a>> for BoxrType<'a> {
    fn from_iter<T: IntoIterator<Item = BoxrType<'a>>>(iter: T) -> Self {
        let stack: Vec<BoxrType<'a>> = iter.into_iter().collect();
        let mut current = BoxrType::NIL;
        for item in stack.into_iter().rev() {
            current = BoxrType::cons_cell(item, current);
        }
        current
    }
}

impl<'a> Iterator for BoxrType<'a> {
    type Item = BoxrType<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.top() {
            Some((car, cdr)) => {
                *self = cdr;
                Some(car)
            },
            None => None,
        }
    }
}

impl<'a> Display for BoxrType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxrType::NIL => write!(f, "'()"),
            BoxrType::CELL(car, cdr) => {
                let mut result = String::from(format!("({}", car));
                for elem in *cdr.clone().into_iter() {
                    result = format!("{} {}", result, elem);
                }
                write!(f, "{})", result)
            },
            BoxrType::INT(i) => write!(f, "{}", i),
            BoxrType::STR(s) => write!(f, "{}", s),
        }
    }
}

impl<'a> Debug for BoxrType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxrType::NIL => write!(f, "NIL"),
            BoxrType::CELL(car, cdr) => write!(f, "(cons {:?} {:?})", car, cdr),
            BoxrType::INT(i) => write!(f, "{}", i),
            BoxrType::STR(s) => write!(f, "{}", s),
        }
    }
}
