use crate::errors::EvaluatorError;

use super::{
    builtin::{BuiltinFunction, BuiltinMacro},
    scope::LexicalVarStorage,
    userfunctions::UserDefinedFunction,
    Expr, Value,
};

pub enum Function {
    UserDefined(UserDefinedFunction),
    Builtin(BuiltinFunction),
    Macro(BuiltinMacro),
}

impl TryFrom<Expr> for Function {
    type Error = EvaluatorError;
    fn try_from(expr: Expr) -> Result<Function, EvaluatorError> {
        match expr {
            Expr::Value(v) => match v {
                Value::Symbol(s) => match BuiltinFunction::try_from(s.as_str()) {
                    Ok(builtin) => Ok(Function::Builtin(builtin)),
                    Err(_) => match BuiltinMacro::try_from(s.as_str()) {
                        Ok(builtin) => Ok(Function::Macro(builtin)),
                        Err(_) => Err(EvaluatorError::UndefinedSymbol(format!(
                            "{} is not a defined function.",
                            s
                        ))),
                    },
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
    fn call(&self, args: Vec<Expr>, stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError>;
}

impl CallFunction for Function {
    fn call(&self, args: Vec<Expr>, stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError> {
        match &self {
            Function::Builtin(builtin) => builtin.call(args, stg),
            Function::Macro(builtin_macro) => builtin_macro.call(args, stg),
            Function::UserDefined(userdefined) => userdefined.call(args, stg),
        }
    }
}
