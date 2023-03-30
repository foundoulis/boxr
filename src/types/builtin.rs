use crate::errors::EvaluatorError;
use crate::evaluator::lisp_eval;
use crate::types::userfunctions::UserDefinedFunction;

use super::function::CallFunction;
use super::scope::LexicalVarStorage;
use super::{Expr, Value};
use lazy_static::lazy_static;
use log;
use std::collections::{HashMap, HashSet};

lazy_static! {
    pub static ref BUILTINS_FUNC_MAP: HashMap<&'static str, BuiltinFunction> = {
        let mut m = HashMap::new();
        m.insert("+", BuiltinFunction::Add);
        m.insert("-", BuiltinFunction::Sub);
        m.insert("*", BuiltinFunction::Mul);
        m.insert("/", BuiltinFunction::Div);
        m.insert("floordiv", BuiltinFunction::FloorDiv);
        m.insert("%", BuiltinFunction::Mod);
        m.insert("^", BuiltinFunction::Pow);
        m.insert("=", BuiltinFunction::Eq);
        m.insert("!=", BuiltinFunction::Neq);
        m.insert("<", BuiltinFunction::Lt);
        m.insert(">", BuiltinFunction::Gt);
        m.insert("<=", BuiltinFunction::Lte);
        m.insert(">=", BuiltinFunction::Gte);
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
        m.insert("quote", BuiltinFunction::Quote);
        m.insert("quasiquote", BuiltinFunction::Quasiquote);
        m.insert("unquote", BuiltinFunction::Unquote);
        m.insert("unquote-splicing", BuiltinFunction::UnquoteSplicing);
        m.insert("apply", BuiltinFunction::Apply);
        m.insert("load", BuiltinFunction::Load);
        m.insert("exit", BuiltinFunction::Exit);
        m.insert("help", BuiltinFunction::Help);
        m
    };
    pub static ref BUILTINS_FUNC_SET: HashSet<&'static str> =
        BUILTINS_MACRO_MAP.keys().cloned().collect();
    pub static ref BUILTINS_MACRO_MAP: HashMap<&'static str, BuiltinMacro> = {
        let mut m = HashMap::new();
        m.insert("match", BuiltinMacro::Match);
        m.insert("define", BuiltinMacro::Define);
        m.insert("set", BuiltinMacro::Set);
        m.insert("let", BuiltinMacro::Let);
        m.insert("if", BuiltinMacro::If);
        m.insert("lambda", BuiltinMacro::Lambda);
        m.insert("cond", BuiltinMacro::Cond);
        m.insert("and", BuiltinMacro::And);
        m.insert("or", BuiltinMacro::Or);
        m.insert("eval", BuiltinMacro::Eval);
        m.insert("parse", BuiltinMacro::Parse);
        m
    };
    pub static ref BUILTINS_MACRO_SET: HashSet<&'static str> =
        BUILTINS_MACRO_MAP.keys().cloned().collect();
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
    Quote,
    Quasiquote,
    Unquote,
    UnquoteSplicing,
    Apply,
    Load,
    Exit,
    Help,
}

impl TryFrom<&str> for BuiltinFunction {
    type Error = EvaluatorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match BUILTINS_FUNC_MAP.get(value) {
            Some(builtin) => Ok(*builtin),
            None => Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a builtin function",
                value
            ))),
        }
    }
}

impl CallFunction for BuiltinFunction {
    fn call(
        &self,
        raw_args: Vec<Expr>,
        stg: &mut LexicalVarStorage,
    ) -> Result<Expr, EvaluatorError> {
        // This is a function so everything in the args should be known and evaluated.
        let mut args = Vec::new();
        for arg in &raw_args {
            args.push(lisp_eval(&arg, stg)?);
        }

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
                Ok(Expr::Value(Value::Float(sum)))
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
                Ok(Expr::Value(Value::Float(sum)))
            }
            BuiltinFunction::Mul => {
                let mut product = 1.0;
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => product *= i as f64,
                            Value::Float(fl) => product *= fl,
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Float(product)))
            }
            BuiltinFunction::Div => {
                let mut product = 1.0;
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Expr::Value(value) => match *value {
                            Value::Int(int) => {
                                if i == 0 {
                                    product = int as f64;
                                } else {
                                    product /= int as f64;
                                }
                            }
                            Value::Float(fl) => {
                                if i == 0 {
                                    product = fl;
                                } else {
                                    product /= fl;
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Float(product)))
            }
            BuiltinFunction::FloorDiv => {
                let mut product = 1.0;
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Expr::Value(value) => match *value {
                            Value::Int(int) => {
                                if i == 0 {
                                    product = int as f64;
                                } else {
                                    product /= int as f64;
                                }
                            }
                            Value::Float(fl) => {
                                if i == 0 {
                                    product = fl;
                                } else {
                                    product /= fl;
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Float(product.floor())))
            }
            BuiltinFunction::Mod => {
                let mut product = 1.0;
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Expr::Value(value) => match *value {
                            Value::Int(int) => {
                                if i == 0 {
                                    product = int as f64;
                                } else {
                                    product %= int as f64;
                                }
                            }
                            Value::Float(fl) => {
                                if i == 0 {
                                    product = fl;
                                } else {
                                    product %= fl;
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Float(product)))
            }
            BuiltinFunction::Pow => {
                let mut product = 1.0;
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Expr::Value(value) => match *value {
                            Value::Int(int) => {
                                if i == 0 {
                                    product = int as f64;
                                } else {
                                    product = product.powf(int as f64);
                                }
                            }
                            Value::Float(fl) => {
                                if i == 0 {
                                    product = fl;
                                } else {
                                    product = product.powf(fl);
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Float(product)))
            }
            BuiltinFunction::Eq => {
                let mut first = true;
                let mut last = None;
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => {
                                if first {
                                    last = Some(i);
                                    first = false;
                                } else {
                                    if last != Some(i) {
                                        return Ok(Expr::Value(Value::Boolean(false)));
                                    }
                                }
                            }
                            Value::Float(fl) => {
                                if first {
                                    last = Some(fl as i64);
                                    first = false;
                                } else {
                                    if last != Some(fl as i64) {
                                        return Ok(Expr::Value(Value::Boolean(false)));
                                    }
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Boolean(true)))
            }
            BuiltinFunction::Neq => {
                let mut first = true;
                let mut last = None;
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => {
                                if first {
                                    last = Some(i);
                                    first = false;
                                } else {
                                    if last != Some(i) {
                                        return Ok(Expr::Value(Value::Boolean(true)));
                                    }
                                }
                            }
                            Value::Float(fl) => {
                                if first {
                                    last = Some(fl as i64);
                                    first = false;
                                } else {
                                    if last != Some(fl as i64) {
                                        return Ok(Expr::Value(Value::Boolean(true)));
                                    }
                                }
                            }
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::Boolean(false)))
            }
            BuiltinFunction::Lt => {
                let mut prev = args.get(0).unwrap();
                for arg in args.iter().skip(1) {
                    match (prev, arg) {
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Int(i2))) => {
                            if i1 >= i2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Float(f2))) => {
                            if *i1 as f64 >= *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Int(i2))) => {
                            if *f1 >= *i2 as f64 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Float(f2))) => {
                            if *f1 >= *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        _ => todo!(),
                    };
                    prev = arg;
                }
                Ok(Expr::Value(Value::Boolean(true)))
            }
            BuiltinFunction::Gt => {
                let mut prev = args.get(0).unwrap();
                for arg in args.iter().skip(1) {
                    match (prev, arg) {
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Int(i2))) => {
                            if i1 <= i2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Float(f2))) => {
                            if *i1 as f64 <= *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Int(i2))) => {
                            if *f1 <= *i2 as f64 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Float(f2))) => {
                            if *f1 <= *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        _ => todo!(),
                    };
                    prev = arg;
                }
                Ok(Expr::Value(Value::Boolean(true)))
            }
            BuiltinFunction::Lte => {
                let mut prev = args.get(0).unwrap();
                for arg in args.iter().skip(1) {
                    match (prev, arg) {
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Int(i2))) => {
                            if i1 > i2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Float(f2))) => {
                            if *i1 as f64 > *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Int(i2))) => {
                            if *f1 > *i2 as f64 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Float(f2))) => {
                            if *f1 > *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        _ => todo!(),
                    };
                    prev = arg;
                }
                Ok(Expr::Value(Value::Boolean(true)))
            }
            BuiltinFunction::Gte => {
                let mut prev = args.get(0).unwrap();
                for arg in args.iter().skip(1) {
                    match (prev, arg) {
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Int(i2))) => {
                            if i1 < i2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Int(i1)), Expr::Value(Value::Float(f2))) => {
                            if (*i1 as f64) < *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Int(i2))) => {
                            if *f1 < *i2 as f64 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        (Expr::Value(Value::Float(f1)), Expr::Value(Value::Float(f2))) => {
                            if *f1 < *f2 {
                                return Ok(Expr::Value(Value::Boolean(false)));
                            }
                        }
                        _ => todo!(),
                    };
                    prev = arg;
                }
                Ok(Expr::Value(Value::Boolean(true)))
            }
            BuiltinFunction::Print => {
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => print!("{}", i),
                            Value::Float(fl) => print!("{}", fl),
                            Value::String(s) => print!("{}", s),
                            Value::Boolean(b) => print!("{}", b),
                            Value::NIL => print!("None"),
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::NIL))
            }
            BuiltinFunction::Println => {
                for arg in args {
                    match arg {
                        Expr::Value(value) => match value {
                            Value::Int(i) => println!("{}", i),
                            Value::Float(fl) => println!("{}", fl),
                            Value::String(s) => println!("{}", s),
                            Value::Boolean(b) => println!("{}", b),
                            Value::NIL => println!("None"),
                            _ => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                Ok(Expr::Value(Value::NIL))
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuiltinMacro {
    Match,
    Define,
    Lambda,
    Let,
    If,
    Cond,
    And,
    Or,
    Set,
    Eval,
    Parse,
}

impl TryFrom<&str> for BuiltinMacro {
    type Error = EvaluatorError;

    fn try_from(value: &str) -> Result<BuiltinMacro, EvaluatorError> {
        match BUILTINS_MACRO_MAP.get(value) {
            Some(builtin) => Ok(*builtin),
            None => Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a builtin macro",
                value
            ))),
        }
    }
}

impl CallFunction for BuiltinMacro {
    fn call(&self, args: Vec<Expr>, stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError> {
        match self {
            BuiltinMacro::Define => {
                log::debug!("Defining function: {:?}", args);
                match &args[0] {
                    Expr::Value(Value::Symbol(s)) => {
                        // evalute the second arg
                        let second = lisp_eval(&args[1], stg)?;
                        // set the variable
                        log::debug!("Setting variable {} to {:?}", s, second);
                        stg.put(s.as_str(), second);
                    }
                    Expr::List(list) => {
                        log::debug!("Defining with args: {:?}", list);
                        let list_raw = list
                            .iter()
                            .map(|elem| Expr::clone(elem))
                            .collect::<Vec<_>>();

                        let (name_raw, args_raw): (&Expr, &[Expr]) =
                            list_raw.split_first().unwrap();

                        let name = match name_raw {
                            Expr::Value(Value::Symbol(s)) => s.clone(),
                            _ => {
                                return Err(EvaluatorError::BadFunctionDefinition(
                                    "First arg cannot be a quoted list.".to_string(),
                                ))
                            }
                        };

                        let local_args = args_raw
                            .iter()
                            .map(|arg| match arg {
                                Expr::Value(Value::Symbol(s)) => {
                                    Ok(Expr::Value(Value::Symbol(s.clone())))
                                }
                                _ => {
                                    return Err(EvaluatorError::BadFunctionDefinition(
                                        "Args must be symbols.".to_string(),
                                    ))
                                }
                            })
                            .collect::<Result<Vec<_>, EvaluatorError>>()?;

                        let body = &args[1..]
                            .into_iter()
                            .map(|e| Expr::clone(e))
                            .collect::<Vec<_>>();

                        log::debug!(
                            "Defining function {} with args {:?} and body {:?}",
                            name,
                            local_args,
                            body
                        );
                        stg.put_func(&name, UserDefinedFunction::new(local_args, body.to_vec()));
                    }
                    _ => {
                        return Err(EvaluatorError::BadFunctionDefinition(
                            "First arg cannot be a quoted list.".to_string(),
                        ))
                    }
                };

                Ok(Expr::Value(Value::NIL))
            }
            _ => todo!(),
        }
    }
}
