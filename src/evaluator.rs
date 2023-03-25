use crate::{
    errors::EvaluatorError,
    scope::LexicalVarStorage,
    types::{Expr, Value},
};

pub fn lisp_eval(expr: &Expr, stg: LexicalVarStorage) -> Result<Expr, EvaluatorError> {
    match expr {
        Expr::Value(value) => match value {
            Value::NIL => Ok(Expr::Value(Value::NIL)),
            Value::Symbol(s) => Ok(Expr::Value(stg.get(&s).unwrap().clone())),
            Value::String(s) => Ok(Expr::Value(Value::String(s.clone()))),
            Value::Boolean(b) => Ok(Expr::Value(Value::Boolean(*b))),
            Value::Int(i) => Ok(Expr::Value(Value::Int(*i))),
            Value::Float(fl) => Ok(Expr::Value(Value::Float(*fl))),
            Value::Quoted(q) => Ok(Expr::Value(Value::Quoted(q.clone()))),
            Value::Comment(_s) => Ok(Expr::Value(Value::NIL)),
        },
        Expr::List(list) => {
            let output = lisp_eval(list[0].as_ref(), stg.fork());
            if let Ok(_valid) = output {
                unimplemented!()
            } else {
                Err(EvaluatorError("Invalid function call."))
            }
        }
        Expr::QuotedList(_list) => unimplemented!(),
    }
}
