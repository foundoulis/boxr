use crate::slyther::ExprsParser;
use lalrpop_util::lalrpop_mod;
use linefeed::Interface;

lalrpop_mod!(pub slyther);

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
    let reader = Interface::new("boxr").unwrap();
    reader.set_prompt("==> ").unwrap();
    let parser = ExprsParser::new();
    loop {
        match reader.read_line().unwrap() {
            linefeed::ReadResult::Input(line) => {
                let expr = parser.parse(&line).unwrap();
                println!("{:?}", expr);
            }
            linefeed::ReadResult::Eof => break,
            linefeed::ReadResult::Signal(_) => break,
        }
    }
}
