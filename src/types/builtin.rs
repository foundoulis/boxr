use super::function::CallFunction;
use super::{Expr, Value};
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    pub static ref BUILTINS_MAP: HashMap<&'static str, BuiltinFunction> = {
        let mut m = HashMap::new();
        m.insert("+", BuiltinFunction::Add);
        m.insert("-", BuiltinFunction::Sub);
        m.insert("*", BuiltinFunction::Mul);
        m.insert("/", BuiltinFunction::Div);
        m.insert("floor/", BuiltinFunction::FloorDiv);
        m.insert("%", BuiltinFunction::Mod);
        m.insert("^", BuiltinFunction::Pow);
        m.insert("=", BuiltinFunction::Eq);
        m.insert("!=", BuiltinFunction::Neq);
        m.insert("<", BuiltinFunction::Lt);
        m.insert(">", BuiltinFunction::Gt);
        m.insert("<=", BuiltinFunction::Lte);
        m.insert(">=", BuiltinFunction::Gte);
        m.insert("and", BuiltinFunction::And);
        m.insert("or", BuiltinFunction::Or);
        m.insert("not", BuiltinFunction::Not);
        m.insert("print", BuiltinFunction::Print);
        m.insert("println", BuiltinFunction::Println);
        m.insert("input", BuiltinFunction::Input);
        m.insert("list", BuiltinFunction::List);
        m.insert("cons", BuiltinFunction::Cons);
        m.insert("car", BuiltinFunction::Car);
        m.insert("cdr", BuiltinFunction::Cdr);
        m.insert("is-list", BuiltinFunction::IsList);
        m.insert("is-symbol", BuiltinFunction::IsSymbol);
        m.insert("is-string", BuiltinFunction::IsString);
        m.insert("is-boolean", BuiltinFunction::IsBoolean);
        m.insert("is-int", BuiltinFunction::IsInt);
        m.insert("is-float", BuiltinFunction::IsFloat);
        m.insert("is-quoted", BuiltinFunction::IsQuoted);
        m.insert("is-comment", BuiltinFunction::IsComment);
        m.insert("is-function", BuiltinFunction::IsFunction);
        m.insert("is-macro", BuiltinFunction::IsMacro);
        m.insert("is-nil", BuiltinFunction::IsNil);
        m.insert("is-defined", BuiltinFunction::IsDefined);
        m.insert("is-bound", BuiltinFunction::IsBound);
        m.insert("define", BuiltinFunction::Define);
        m.insert("set", BuiltinFunction::Set);
        m.insert("let", BuiltinFunction::Let);
        m.insert("if", BuiltinFunction::If);
        m.insert("quote", BuiltinFunction::Quote);
        m.insert("quasiquote", BuiltinFunction::Quasiquote);
        m.insert("unquote", BuiltinFunction::Unquote);
        m.insert("unquote-splicing", BuiltinFunction::UnquoteSplicing);
        m.insert("lambda", BuiltinFunction::Lambda);
        m.insert("macro", BuiltinFunction::Macro);
        m.insert("eval", BuiltinFunction::Eval);
        m.insert("apply", BuiltinFunction::Apply);
        m.insert("load", BuiltinFunction::Load);
        m.insert("exit", BuiltinFunction::Exit);
        m.insert("help", BuiltinFunction::Help);
        m
    };
    pub static ref BUILTINS_SET: HashSet<&'static str> = BUILTINS_MAP.keys().cloned().collect();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinFunction {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Not,
    Print,
    Println,
    Input,
    List,
    Cons,
    Car,
    Cdr,
    IsList,
    IsSymbol,
    IsString,
    IsBoolean,
    IsInt,
    IsFloat,
    IsQuoted,
    IsComment,
    IsFunction,
    IsMacro,
    IsNil,
    IsDefined,
    IsBound,
    Define,
    Set,
    Let,
    If,
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
    Lambda,
    Macro,
    Eval,
    Apply,
    Load,
    Exit,
    Help,
}

impl TryFrom<&str> for BuiltinFunction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match BUILTINS_MAP.get(value) {
            Some(builtin) => Ok(*builtin),
            None => Err(format!("{} is not a builtin function", value)),
        }
    }
}

impl CallFunction for BuiltinFunction {
    fn call(&self, args: Vec<Expr>) -> Expr {
        match self {
            BuiltinFunction::Add => {
                let mut sum = 0.0;
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => sum += i as f64,
                            Value::Float(fl) => sum += fl,
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Expr::Value(Value::Float(sum))
            }
            BuiltinFunction::Sub => {
                let mut sum = 0.0;
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Expr::Value(value) => match *value {
                            Value::Int(int) => {
                                if i == 0 {
                                    sum = int as f64;
                                } else {
                                    sum -= int as f64;
                                }
                            }
                            Value::Float(fl) => {
                                if i == 0 {
                                    sum = fl;
                                } else {
                                    sum -= fl;
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Expr::Value(Value::Float(sum))
            }
            _ => todo!(),
        }
    }
}
