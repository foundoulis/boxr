use crate::{
    errors::EvaluatorError,
    types::{
        function::{CallFunction, Function},
        scope::LexicalVarStorage,
    },
    types::{Expr, Value},
};
use std::sync::Arc;

pub fn lisp_eval(expr: &Expr, stg: LexicalVarStorage) -> Result<Expr, EvaluatorError> {
    println!("Evaluating: {:?}", expr);
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
                Some(v) => return Ok(Expr::Value(v.clone())),
                None => return Err(EvaluatorError::UndefinedSymbol(s.clone())),
            },
        },
        Expr::List(list) => {
            // Grab the name of the function of the list.
            let first_elem: Expr = Expr::clone(list[0].as_ref());

            // If the first element is not a symbol, return an error.
            if let Expr::Value(Value::Symbol(_)) = first_elem {
            } else {
                return Err(EvaluatorError::NotAFunction(format!(
                    "Type: {} is not callable.",
                    Expr::from(first_elem),
                )));
            }

            // The rest of the list are the arguments, evaluate them.
            let rest_of_list: Vec<Result<Expr, EvaluatorError>> =
                list[1..].iter().map(|a| lisp_eval(a, stg.fork())).collect();

            // If any of the arguments are not Ok, return the first error.
            if rest_of_list.iter().any(|a| a.is_err()) {
                return Err(rest_of_list
                    .iter()
                    .find(|a| a.is_err())
                    .unwrap()
                    .as_ref()
                    .unwrap_err()
                    .clone());
            }

            // Unwrap all the arguments into Exprs.
            let arguments: Vec<Expr> = rest_of_list
                .iter()
                .map(|a| Expr::clone((*a).as_ref().unwrap()))
                .collect();

            // We can now call the function.
            return Ok(Function::try_from(first_elem)?.call(arguments));
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
