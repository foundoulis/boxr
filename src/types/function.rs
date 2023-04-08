use crate::{errors::EvaluatorError, evaluator::lisp_eval};

use super::{scope::LexicalVarStorage, Cons, ConsValue};

use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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
        BUILTINS_FUNC_MAP.keys().cloned().collect();
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

impl BuiltinFunction {
    pub fn get(symbol: &Cons) -> Option<Self> {
        if let Cons::Value(ConsValue::Symbol(s)) = symbol {
            BUILTINS_FUNC_MAP.get(s.as_str()).copied()
        } else {
            None
        }
    }
    pub fn call(&self, args: Vec<Cons>) -> Result<Cons, EvaluatorError> {
        match *self {
            BuiltinFunction::Add => {
                let mut sum = 0.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum += f;
                        } else {
                            sum += f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum += *i as f64;
                        } else {
                            sum += *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for +".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::Sub => {
                if args.len() == 1 {
                    if let Cons::Value(ConsValue::Float(f)) = &args[0] {
                        return Ok(Cons::Value(ConsValue::Float(-f)));
                    } else if let Cons::Value(ConsValue::Int(i)) = &args[0] {
                        return Ok(Cons::Value(ConsValue::Int(-i)));
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for -".to_string(),
                        ));
                    }
                }
                let mut sum = 0.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum += f;
                        } else {
                            sum -= f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum += *i as f64;
                        } else {
                            sum -= *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for -".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::Mul => {
                let mut sum = 1.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum *= f;
                        } else {
                            sum *= f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum *= *i as f64;
                        } else {
                            sum *= *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for *".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::Div => {
                if args.len() == 1 {
                    if let Cons::Value(ConsValue::Int(i)) = &args[0] {
                        return Ok(Cons::Value(ConsValue::Float(1.0 / *i as f64)));
                    } else if let Cons::Value(ConsValue::Float(f)) = &args[0] {
                        return Ok(Cons::Value(ConsValue::Float(1.0 / *f)));
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for /".to_string(),
                        ));
                    }
                }
                let mut sum = 1.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum *= f;
                        } else {
                            sum /= f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum *= *i as f64;
                        } else {
                            sum /= *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for /".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::FloorDiv => {
                let mut sum = 1.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum *= f;
                        } else {
                            sum /= f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum *= *i as f64;
                        } else {
                            sum /= *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for //".to_string(),
                        ));
                    }
                }
                Ok(Cons::Value(ConsValue::Int(sum as i64)))
            }
            BuiltinFunction::Mod => {
                let mut sum = 1.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum *= f;
                        } else {
                            sum %= f;
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum *= *i as f64;
                        } else {
                            sum %= *i as f64;
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for %".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::Pow => {
                let mut sum = 1.0;
                for (index, arg) in args.iter().enumerate() {
                    if let Cons::Value(ConsValue::Float(f)) = arg {
                        if index == 0 {
                            sum *= f;
                        } else {
                            sum = sum.powf(*f);
                        }
                    } else if let Cons::Value(ConsValue::Int(i)) = arg {
                        if index == 0 {
                            sum *= *i as f64;
                        } else {
                            sum = sum.powf(*i as f64);
                        }
                    } else {
                        return Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for ^".to_string(),
                        ));
                    }
                }
                if sum.fract() == 0.0 {
                    Ok(Cons::Value(ConsValue::Int(sum as i64)))
                } else {
                    Ok(Cons::Value(ConsValue::Float(sum)))
                }
            }
            BuiltinFunction::Eq => Ok(Cons::Value(ConsValue::Boolean(
                args.iter().all(|arg| arg == &args[0]),
            ))),
            BuiltinFunction::Neq => Ok(Cons::Value(ConsValue::Boolean(
                args.iter().any(|arg| arg != &args[0]),
            ))),
            _ => Ok(Cons::Value(ConsValue::NIL)),
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

pub enum MacroReturn {
    None,
    Value(Cons),
    Function(UserFunction),
}

impl BuiltinMacro {
    #[mutants::skip]
    pub fn get(symbol: &Cons) -> Option<Self> {
        if let Cons::Value(ConsValue::Symbol(s)) = symbol {
            BUILTINS_MACRO_MAP.get(s.as_str()).copied()
        } else {
            None
        }
    }
    pub fn call(
        &self,
        args: &Cons,
        stg: &mut LexicalVarStorage,
    ) -> Result<MacroReturn, EvaluatorError> {
        match *self {
            BuiltinMacro::Define => {
                let (name, body) = args.split().unwrap();
                // Here we execute (define x eval-part)
                // this is both variables and lambda functions (for free)
                if let Cons::Value(ConsValue::Symbol(s)) = name {
                    let result = lisp_eval(&body, stg)?;
                    stg.put(&s, result.clone());
                    Ok(MacroReturn::Value(Cons::Value(ConsValue::NIL)))
                // Here we execute (define (func-name arg1 ...) (body1) ...)
                } else if let Some((car, cdr)) = name.split() {
                    if let Cons::Value(ConsValue::Symbol(s)) = car {
                        let result = UserFunction::new(cdr.clone(), body.clone(), stg.fork());
                        stg.put_func(&s, result);
                        Ok(MacroReturn::Value(Cons::Value(ConsValue::NIL)))
                    } else {
                        Err(EvaluatorError::InvalidArgument(
                            "Invalid argument type for define".to_string(),
                        ))
                    }
                // Here the definition is poorly designed and doesn't work
                } else {
                    Err(EvaluatorError::InvalidArgument(
                        "Invalid argument type for define".to_string(),
                    ))
                }
            }
            BuiltinMacro::Lambda => {
                let (args, body) = args.split().unwrap();
                Ok(MacroReturn::Function(UserFunction::new(
                    args.clone(),
                    body.clone(),
                    stg.fork(),
                )))
            }
            _ => Err(EvaluatorError::UndefinedSymbol(
                "Undefined symbol".to_string(),
            )),
        }
    }
}
#[derive(Debug, Clone)]
pub struct UserFunction {
    args: Cons,
    body: Cons,
    environ: LexicalVarStorage,
}

impl UserFunction {
    pub fn new(args: Cons, body: Cons, environ: LexicalVarStorage) -> Self {
        Self {
            args,
            body,
            environ,
        }
    }
    pub fn call(&self, args: Cons) -> Result<Cons, EvaluatorError> {
        let args: Vec<Cons> = args.into_iter().collect();
        let mut combined_environment = self.environ.fork();
        for (index, elem) in Cons::clone(&self.args).into_iter().enumerate() {
            if let Cons::Value(ConsValue::Symbol(s)) = elem {
                combined_environment.put(&s, args[index].clone());
            }
        }
        log::debug!("Calling function with args: {:?}", args);
        log::debug!("Calling function with environ: {:?}", combined_environment);
        let mut result = Cons::Value(ConsValue::NIL);
        for i in Cons::clone(&self.body) {
            log::debug!("Evaluating user_func: {}", i);
            result = lisp_eval(&i, &mut combined_environment)?;
        }
        Ok(result)
    }
    pub fn to_cons(&self) -> Cons {
        Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("lambda".to_string())),
            self.args.clone(),
            self.body.clone(),
        ])
    }
}

#[mutants::skip]
impl Display for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(lambda {} {})", self.args, self.body)
    }
}
