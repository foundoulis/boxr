use crate::{
    errors::EvaluatorError,
    types::{
        function::{BuiltinFunction, BuiltinMacro},
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
        Err(e) => Err(e),
    }
}

enum EvalReturnType {
    CONS(Cons),
    FUNC(BuiltinFunction),
    MACRO(BuiltinMacro),
}

fn lisp_eval_int(
    expr: &Cons,
    stg: &mut LexicalVarStorage,
) -> Result<EvalReturnType, EvaluatorError> {
    log::debug!("Evaluating: {:?}", expr);
    log::debug!("Storage: {:?}", stg);
    loop {
        let _expr: Cons = match expr {
            Cons::Quoted(q) => return Ok(EvalReturnType::CONS(Cons::clone(q))),
            Cons::Value(value) => match value {
                ConsValue::NIL => return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::NIL))),
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

                    // Then look for variables.
                    else {
                        return Ok(EvalReturnType::CONS(stg[s].clone()));
                    }
                }
                ConsValue::String(s) => {
                    return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::String(
                        s.clone(),
                    ))))
                }
                ConsValue::Boolean(b) => {
                    return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Boolean(*b))))
                }
                ConsValue::Int(i) => {
                    return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Int(*i))))
                }
                ConsValue::Float(fl) => {
                    return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Float(*fl))))
                }
                ConsValue::Comment(s) => {
                    return Ok(EvalReturnType::CONS(Cons::Value(ConsValue::Comment(
                        s.clone(),
                    ))))
                }
            },
            Cons::Cell(car, _cdr) => match lisp_eval_int(car, stg)? {
                EvalReturnType::CONS(c) => return Ok(EvalReturnType::CONS(c)),
                EvalReturnType::MACRO(m) => {
                    return Ok(EvalReturnType::CONS(m.call(&expr.cdr(), &mut stg.fork())?));
                }
                EvalReturnType::FUNC(f) => {
                    // All functions eval their args before they start.
                    let evaled_args: Result<Vec<Cons>, EvaluatorError> =
                        expr.cdr().into_iter().map(|c| lisp_eval(&c, stg)).collect();
                    return Ok(EvalReturnType::CONS(f.call(evaled_args?)?));
                }
            },
        };
    }
}
