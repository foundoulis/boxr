use crate::{
    errors::EvaluatorError,
    types::{
        function::{CallFunction, Function},
        scope::LexicalVarStorage,
    },
    types::{Expr, Value},
};
use std::sync::Arc;

pub fn lisp_eval(expr: &Expr, stg: &mut LexicalVarStorage) -> Result<Expr, EvaluatorError> {
    log::debug!("Evaluating: {:?}", expr);
    log::debug!("Storage: {:?}", stg);
    match expr {
        Expr::Value(value) => match value {
            Value::NIL => return Ok(Expr::Value(Value::NIL)),
            Value::String(s) => return Ok(Expr::Value(Value::String(s.clone()))),
            Value::Boolean(b) => return Ok(Expr::Value(Value::Boolean(*b))),
            Value::Int(i) => return Ok(Expr::Value(Value::Int(*i))),
            Value::Float(fl) => return Ok(Expr::Value(Value::Float(*fl))),
            Value::Quoted(q) => return Ok(Expr::Value(Value::clone(&q))),
            Value::Comment(_s) => return Ok(Expr::Value(Value::NIL)),
            Value::Symbol(s) => match stg.get(&s) {
                Some(v) => return Ok(Expr::clone(v)),
                None => return Err(EvaluatorError::UndefinedSymbol(s.clone())),
            },
        },
        Expr::List(list) => {
            // Grab the name of the function of the list.
            let first_elem: Expr = Expr::clone(list[0].as_ref());

            // If the first element is not a symbol, return an error.
            if let Expr::Value(Value::Symbol(ref name)) = first_elem {
                // Look for builtin functions
                if let Ok(function) = Function::get_function(first_elem.clone(), stg) {
                    // Unwrap all the arguments into Exprs.
                    let mut arguments: Vec<Expr> = Vec::new();
                    for elem in &list[1..] {
                        arguments.push(elem.as_ref().clone());
                    }

                    function.call(arguments, stg)

                // Look for user defined functions
                } else if let Some(function) = stg.get_func(&name) {
                    // Unwrap all the arguments into Exprs.
                    let mut arguments: Vec<Expr> = Vec::new();
                    for elem in &list[1..] {
                        arguments.push(elem.as_ref().clone());
                    }

                    function.call(arguments, &mut stg.fork())
                } else {
                    return Err(EvaluatorError::UndefinedSymbol(name.clone()));
                }
            } else {
                return Err(EvaluatorError::NotAFunction(format!(
                    "Type: {} is not callable.",
                    Expr::from(first_elem),
                )));
            }
        }
        Expr::QuotedList(list) => {
            let reg_list = Expr::List(
                list.iter()
                    .map(|a| match a.as_ref() {
                        Expr::Value(v) => Arc::new(Expr::Value(Value::Quoted(Arc::new(v.clone())))),
                        Expr::List(l) => Arc::new(Expr::QuotedList(l.clone())),
                        Expr::QuotedList(l) => Arc::new(Expr::QuotedList(l.clone())),
                    })
                    .collect(),
            );
            return Ok(reg_list);
        }
    }
}
