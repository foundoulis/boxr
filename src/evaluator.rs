use crate::{
    errors::EvaluatorError,
    types::{
        function::{BuiltinFunction, BuiltinMacro, MacroReturn, UserFunction},
        scope::LexicalVarStorage,
    },
    types::{Cons, ConsValue},
};

pub fn lisp_eval(expr: &Cons, stg: &mut LexicalVarStorage) -> Result<Cons, EvaluatorError> {
    match lisp_eval_int(expr, stg) {
        Ok(EvalReturnType::CONS(c)) => Ok(c),
        Ok(EvalReturnType::FUNC(f)) => Err(EvaluatorError::ReturnedNonCons(format!(
            "Function {:?} returned non-cons value",
            f
        ))),
        Ok(EvalReturnType::MACRO(m)) => Err(EvaluatorError::ReturnedNonCons(format!(
            "Macro {:?} returned non-cons value",
            m
        ))),
        Ok(EvalReturnType::USER(u)) => Ok(u.to_cons()),
        Err(e) => Err(e),
    }
}

enum EvalReturnType {
    CONS(Cons),
    FUNC(BuiltinFunction),
    MACRO(BuiltinMacro),
    USER(UserFunction),
}

fn lisp_eval_int(
    expr: &Cons,
    stg: &mut LexicalVarStorage,
) -> Result<EvalReturnType, EvaluatorError> {
    log::debug!("Evaluating: {:?}", expr);
    log::debug!("Storage: {:?}", stg);
    match expr {
        Cons::Quoted(q) => Ok(EvalReturnType::CONS(Cons::clone(q))),
        Cons::Value(value) => match value {
            ConsValue::NIL => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::NIL))),
            ConsValue::Symbol(s) => {
                // First look for builtin functions.
                if let Some(builtin_func) = BuiltinFunction::get(expr) {
                    return Ok(EvalReturnType::FUNC(builtin_func));
                }
                // Then look for builtin macros.
                else if let Some(builtin_macro) = BuiltinMacro::get(expr) {
                    return Ok(EvalReturnType::MACRO(builtin_macro));
                }
                // Then look for user-defined functions.
                else if let Some(user_func) = stg.get_func(s) {
                    return Ok(EvalReturnType::USER(user_func.clone()));
                }
                // Then look for variables.
                else {
                    return Ok(EvalReturnType::CONS(stg[s].clone()));
                }
            }
            ConsValue::String(s) => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::String(
                s.clone(),
            )))),
            ConsValue::Boolean(b) => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Boolean(*b)))),
            ConsValue::Int(i) => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Int(*i)))),
            ConsValue::Float(fl) => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Float(*fl)))),
            ConsValue::Comment(s) => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Comment(
                s.clone(),
            )))),
        },
        Cons::Cell(car, _cdr) => match lisp_eval_int(car, stg)? {
            EvalReturnType::CONS(c) => Ok(EvalReturnType::CONS(c)),
            EvalReturnType::MACRO(m) => match m.call(&expr.cdr(), stg)? {
                MacroReturn::Value(c) => Ok(EvalReturnType::CONS(c)),
                MacroReturn::Function(f) => Ok(EvalReturnType::USER(f)),
                MacroReturn::None => Ok(EvalReturnType::CONS(Cons::Value(ConsValue::NIL))),
            },
            EvalReturnType::FUNC(f) => {
                // All builtin functions eval their args before they start.
                let evaled_args: Result<Vec<Cons>, EvaluatorError> = expr
                    .cdr()
                    .into_iter()
                    .map(|c| lisp_eval(&c, &mut stg.fork()))
                    .collect();
                Ok(EvalReturnType::CONS(f.call(evaled_args?)?))
            }
            EvalReturnType::USER(f) => {
                Ok(EvalReturnType::CONS(f.call(expr.cdr(), &mut stg.fork())?))
            }
        },
    }
}
