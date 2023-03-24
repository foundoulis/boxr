use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub slyther);
use crate::slyther::ExprsParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Value(Value),
    List(Vec<Box<Expr>>),
    QuotedList(Vec<Box<Expr>>),
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

fn main() {
    let input = r#"(8 1 (- 2 3) (* 4.0 5) '(6 7 8) #t #f "stringy" '()) ; comment"#;
    let parser = ExprsParser::new();
    let expr = parser.parse(input).unwrap();
    println!("{:?}", expr);
}
