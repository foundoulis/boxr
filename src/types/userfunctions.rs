use crate::errors::EvaluatorError;

use super::{function::CallFunction, scope::LexicalVarStorage, Expr};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFunction {
    pub args: Vec<Expr>,
    pub body: Vec<Expr>,
}

impl CallFunction for UserDefinedFunction {
    fn call(&self, _args: Vec<Expr>, _stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError> {
        todo!()
    }
}

impl UserDefinedFunction {
    pub fn new(args: Vec<Expr>, body: Vec<Expr>) -> UserDefinedFunction {
        UserDefinedFunction { args, body }
    }
}
