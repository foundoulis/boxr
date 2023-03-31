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

impl Function {
    pub fn get_function(
        expr: Expr,
        stg: &mut LexicalVarStorage,
    ) -> Result<Function, EvaluatorError> {
        if let Expr::Value(Value::Symbol(sym)) = expr {
            if let Ok(builtin) = BuiltinFunction::try_from(sym.as_str()) {
                Ok(Function::Builtin(builtin))
            } else if let Ok(builtin_macro) = BuiltinMacro::try_from(sym.as_str()) {
                Ok(Function::Macro(builtin_macro))
            } else if let Ok(user_defined) = UserDefinedFunction::get_func(sym.as_str(), stg) {
                Ok(Function::UserDefined(user_defined))
            } else {
                Err(EvaluatorError::UndefinedSymbol(format!(
                    "{} is not a defined function.",
                    sym
                )))
            }
        } else {
            Err(EvaluatorError::UncallableType(format!(
                "{} is not a callable type.",
                expr
            )))
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
