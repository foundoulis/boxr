use crate::errors::EvaluatorError;

use super::{
    builtin::BuiltinFunction, macros::BuiltinMacro, scope::LexicalVarStorage,
    userfunctions::UserDefinedFunction, Expr,
};

pub enum Function {
    Builtin(BuiltinFunction),
    UserDefined(UserDefinedFunction),
    Macro(BuiltinMacro),
}

pub trait CallableFunction {
    fn get(sym: &str, stg: &mut LexicalVarStorage) -> Result<Function, EvaluatorError>;
    fn call(
        &self,
        name: &str,
        args: Vec<Expr>,
        stg: &mut LexicalVarStorage,
    ) -> Result<Expr, EvaluatorError>;
}

impl CallableFunction for Function {
    fn get(sym: &str, stg: &mut LexicalVarStorage) -> Result<Function, EvaluatorError> {
        if let Ok(builtin) = BuiltinFunction::get(sym, stg) {
            Ok(builtin)
        } else if let Ok(builtin_macro) = BuiltinMacro::get(sym, stg) {
            Ok(builtin_macro)
        } else if let Ok(user_defined) = UserDefinedFunction::get(sym, stg) {
            Ok(user_defined)
        } else {
            Err(EvaluatorError::UndefinedSymbol(format!(
                "{} is not a defined function.",
                sym
            )))
        }
    }
    fn call(
        &self,
        name: &str,
        args: Vec<Expr>,
        stg: &mut LexicalVarStorage,
    ) -> Result<Expr, EvaluatorError> {
        match &self {
            Function::Builtin(builtin) => builtin.call(name, args, stg),
            Function::UserDefined(userdefined) => userdefined.call(name, args, stg),
            Function::Macro(builtin_macro) => builtin_macro.call(name, args, stg),
        }
    }
}
