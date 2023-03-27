pub mod errors;
pub mod evaluator;
pub mod types;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub slyther);

#[cfg(test)]
mod test_parsing {
    use super::slyther::ExprsParser;
    use super::types::{Expr, Value};
    use std::sync::Arc;

    #[test]
    fn test_parsing() {
        let input = "(+ 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(
            expr == &Arc::new(Expr::List(vec![
                Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                Arc::new(Expr::Value(Value::Int(1))),
                Arc::new(Expr::Value(Value::Int(2))),
            ]))
        );
    }

    #[test]
    fn test_parsing_nested() {
        let input = "(+ 1 (+ 2 3))";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(
            expr == &Arc::new(Expr::List(vec![
                Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                Arc::new(Expr::Value(Value::Int(1))),
                Arc::new(Expr::List(vec![
                    Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                    Arc::new(Expr::Value(Value::Int(2))),
                    Arc::new(Expr::Value(Value::Int(3))),
                ])),
            ]))
        );
    }
}

#[cfg(test)]
mod test_evaluator_simple {
    use crate::evaluator::lisp_eval;
    use crate::types::scope::LexicalVarStorage;

    use super::slyther::ExprsParser;
    use super::types::{Expr, Value};
    use std::sync::Arc;

    #[test]
    fn test_evaluator_nil() {
        let input = "'()";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::NIL)));
    }

    #[test]
    fn test_evaluator_int() {
        let input = "1";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Int(1))));
    }

    #[test]
    fn test_evaluator_string() {
        let input = "\"1\"";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::String("\"1\"".to_string()))));
    }

    #[test]
    fn test_evaluator_symbol() {
        let input = "a";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Symbol("a".to_string()))));
    }

    #[test]
    fn test_evaluator_bool() {
        let input = "#t";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Boolean(true))));
    }

    #[test]
    fn test_evaluator_float() {
        let input = "1.0";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Float(1.0))));
    }

    #[test]
    fn test_evaluator_comment() {
        let input = ";1";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Comment(";1".to_string()))));
    }

    #[test]
    fn test_evaluator_quote_int() {
        let input = "'1";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = lisp_eval(&parsed_input[0], LexicalVarStorage::new());
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert!(expr == Expr::Value(Value::Int(1)));
    }

    #[test]
    fn test_evaluator_quote_str() {
        let input = "'\"1\"";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = lisp_eval(&parsed_input[0], LexicalVarStorage::new());
        assert!(expr.is_ok());
        let expr = expr.unwrap();
        assert!(expr == Expr::Value(Value::String("\"1\"".to_string())));
    }

    #[test]
    fn test_evaluator_list() {
        let input = "(+ 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(
            expr == &Arc::new(Expr::List(vec![
                Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                Arc::new(Expr::Value(Value::Int(1))),
                Arc::new(Expr::Value(Value::Int(2))),
            ]))
        );
    }
}

#[cfg(test)]
mod test_evaluator_quotes {
    use crate::evaluator::lisp_eval;
    use crate::slyther::ExprsParser;
    use crate::types::scope::LexicalVarStorage;
    use crate::types::{Expr, Value};
    use std::sync::Arc;

    #[test]
    fn test_quoted_list() {
        let input = "'(1 2 3)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            Expr::List(vec![
                Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Int(1))))),
                Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Int(2))))),
                Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Int(3))))),
            ])
        );
    }

    #[test]
    fn test_quoted_list_nested() {
        let input = "'(1 '2 (3 4))";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            Expr::List(vec![
                Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Int(1))))),
                Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Quoted(
                    Arc::new(Value::Int(2))
                ))))),
                Arc::new(Expr::QuotedList(vec![
                    Arc::new(Expr::Value(Value::Int(3))),
                    Arc::new(Expr::Value(Value::Int(4))),
                ])),
            ])
        );
    }
}

#[cfg(test)]
mod test_evaluator_builtins {
    use crate::{
        evaluator::lisp_eval,
        slyther::ExprsParser,
        types::{scope::LexicalVarStorage, Expr, Value},
    };

    #[test]
    fn test_add_builtin() {
        let input = "(+ 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(3.0)));
    }

    #[test]
    fn test_sub_builtin() {
        let input = "(- 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(-1.0)));
    }

    #[test]
    fn test_mul_builtin() {
        let input = "(* 1 2 3 4)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(24.0)));
    }

    #[test]
    fn test_div_builtin() {
        let input = "(/ 1 2 3 4)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(0.041666666666666664)));
    }

    #[test]
    fn test_floordiv_builtin() {
        // should add more and better tests for this function.
        let input = "(floordiv 1 2 3 4)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(0.0)));
    }
}

#[cfg(test)]
mod test_value_types {
    use super::types::Expr;
    use super::types::Value;
    use std::sync::Arc;

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Int(1)), "1");
        assert_eq!(format!("{}", Value::Symbol("a".to_string())), "a");
        assert_eq!(format!("{}", Value::NIL), "'()");
        assert_eq!(format!("{}", Value::Quoted(Arc::new(Value::Int(1)))), "'1");
    }

    #[test]
    fn test_expr_display() {
        assert_eq!(format!("{}", Expr::Value(Value::Int(1))), "1");
        assert_eq!(
            format!("{}", Expr::Value(Value::Symbol("a".to_string()))),
            "a"
        );
        assert_eq!(format!("{}", Expr::Value(Value::NIL)), "'()");
        assert_eq!(
            format!("{}", Expr::Value(Value::Quoted(Arc::new(Value::Int(1))))),
            "'1"
        );
        assert_eq!(
            format!(
                "{}",
                Expr::List(vec![
                    Arc::new(Expr::Value(Value::Int(1))),
                    Arc::new(Expr::Value(Value::Int(2))),
                ])
            ),
            "(1 2)"
        );
    }
}

#[cfg(test)]
mod test_var_storage {
    use super::types::scope::LexicalVarStorage;
    use super::types::Value;

    #[test]
    fn test_set_get() {
        let mut storage = LexicalVarStorage::new();
        storage.put("a", Value::Int(1));
        assert_eq!(storage.get("a"), Some(&Value::Int(1)));
    }

    #[test]
    fn test_fork() {
        let mut storage = LexicalVarStorage::new();
        storage.put("a", Value::Int(1));
        let mut fork = storage.fork();
        fork.put("b", Value::Int(2));
        assert_eq!(storage.get("a"), Some(&Value::Int(1)));
        assert_eq!(storage.get("b"), None);
        assert_eq!(fork.get("a"), Some(&Value::Int(1)));
        assert_eq!(fork.get("b"), Some(&Value::Int(2)));
    }
}
