use super::{function::CallFunction, Expr};

pub struct UserDefinedFunction;

impl CallFunction for UserDefinedFunction {
    fn call(&self, _args: Vec<Expr>) -> Expr {
        todo!()
    }
}
