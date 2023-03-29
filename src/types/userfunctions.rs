use std::sync::Arc;

use crate::errors::EvaluatorError;

use super::{function::CallFunction, scope::LexicalVarStorage, Expr};

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
    pub name: String,
    pub args: Vec<Arc<Expr>>,
    pub body: Vec<Expr>,
}

impl CallFunction for UserDefinedFunction {
    fn call(&self, _args: Vec<Expr>, _stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError> {
        todo!()
    }
}
