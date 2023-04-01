use crate::{errors::EvaluatorError, evaluator::lisp_eval, types::Value};

use super::{
    function::{CallableFunction, Function},
    scope::LexicalVarStorage,
    Expr,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFunction {
    args: Vec<Expr>,
    body: Vec<Expr>,
}

impl CallableFunction for UserDefinedFunction {
    fn get(sym: &str, stg: &mut LexicalVarStorage) -> Result<Function, EvaluatorError> {
        if let Some(func) = stg.get_func(sym) {
            Ok(Function::UserDefined(func.clone()))
        } else {
            Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a defined function.",
                sym
            )))
        }
    }
    fn call(
        &self,
        _name: &str,
        args: Vec<Expr>,
        stg: &mut LexicalVarStorage,
    ) -> Result<Expr, EvaluatorError> {
        let parsed_args = args
            .iter()
            .map(|arg| lisp_eval(arg, stg))
            .collect::<Result<Vec<Expr>, EvaluatorError>>()?;

        for (arg_name, parsed_args) in self.args.iter().zip(parsed_args.iter()) {
            if let Expr::Value(Value::Symbol(arg_name)) = arg_name {
                stg.put(arg_name, parsed_args.clone());
            } else {
                return Err(EvaluatorError::InvalidArgument(
                    "Invalid argument name.".to_string(),
                ));
            }
        }

        let mut result = Expr::Value(Value::NIL);
        for expr in self.body.iter() {
            result = lisp_eval(expr, &mut stg.fork())?;
        }

        Ok(result)
    }
}

impl UserDefinedFunction {
    pub fn new(args: Vec<Expr>, body: Vec<Expr>) -> UserDefinedFunction {
        UserDefinedFunction { args, body }
    }
    pub fn get_func(
        name: &str,
        stg: &mut LexicalVarStorage,
    ) -> Result<UserDefinedFunction, EvaluatorError> {
        if let Some(func) = stg.get_func(name) {
            Ok(func.clone())
        } else {
            Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a defined function.",
                name
            )))
        }
    }
}

#[mutants::skip]
impl std::fmt::Display for UserDefinedFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(lambda ({:?}) ({:?}))", self.args, self.body)
    }
}
