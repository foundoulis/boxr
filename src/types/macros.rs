use crate::errors::EvaluatorError;
use crate::evaluator::lisp_eval;
use crate::types::userfunctions::UserDefinedFunction;

use super::function::{CallableFunction, Function};
use super::scope::LexicalVarStorage;
use super::{Expr, Value};
use lazy_static::lazy_static;
use log;
use std::collections::{HashMap, HashSet};

lazy_static! {
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

impl CallableFunction for BuiltinMacro {
    fn get(sym: &str, _stg: &mut LexicalVarStorage) -> Result<Function, EvaluatorError> {
        match BUILTINS_MACRO_MAP.get(sym) {
            Some(builtin) => Ok(Function::Macro(*builtin)),
            None => Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a builtin macro",
                sym
            ))),
        }
    }
    fn call(
        &self,
        _name: &str,
        args: Vec<Expr>,
        stg: &mut LexicalVarStorage,
    ) -> Result<Expr, EvaluatorError> {
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
