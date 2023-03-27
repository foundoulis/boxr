use crate::errors::EvaluatorError;

use super::{
    builtin::{BuiltinFunction, BUILTINS_SET},
    userfunctions::UserDefinedFunction,
    Expr, Value,
};

pub enum Function {
    Builtin(BuiltinFunction),
    UserDefined(UserDefinedFunction),
    Macro,
}

impl TryFrom<Expr> for Function {
    type Error = EvaluatorError;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::Value(v) => match v {
                Value::Symbol(s) => match BuiltinFunction::try_from(s.as_str()) {
                    Ok(builtin) => Ok(Function::Builtin(builtin)),
                    Err(_) => Err(EvaluatorError::UndefinedSymbol(format!(
                        "{} is not a defined function.",
                        s
                    ))),
                },
                _ => Err(EvaluatorError::UncallableType(format!(
                    "{} is not a callable type.",
                    v
                ))),
            },
            _ => Err(EvaluatorError::UncallableType(format!(
                "{} is not a callable type.",
                expr
            ))),
        }
    }
}

pub trait CallFunction {
    fn call(&self, args: Vec<Expr>) -> Expr;
}

impl CallFunction for Function {
    fn call(&self, args: Vec<Expr>) -> Expr {
        match &self {
            Function::Builtin(builtin) => builtin.call(args),
            Function::UserDefined(userdefined) => userdefined.call(args),
            Function::Macro => unimplemented!(),
        }
    }
}
