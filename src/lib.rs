pub mod errors;
pub mod evaluator;
pub mod scope;
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
mod test_evaluator {
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
    fn test_evaluator_quote() {
        let input = "'1";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        assert!(expr == &Arc::new(Expr::Value(Value::Quoted(Arc::new(Value::Int(1))))));
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
mod test_types {
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
        assert_eq!(format!("{}", super::types::Expr::Value(Value::Int(1))), "1");
        assert_eq!(
            format!(
                "{}",
                super::types::Expr::Value(Value::Symbol("a".to_string()))
            ),
            "a"
        );
        assert_eq!(format!("{}", super::types::Expr::Value(Value::NIL)), "'()");
        assert_eq!(
            format!(
                "{}",
                super::types::Expr::Value(Value::Quoted(Arc::new(Value::Int(1))))
            ),
            "'1"
        );
        assert_eq!(
            format!(
                "{}",
                super::types::Expr::List(vec![
                    Arc::new(super::types::Expr::Value(Value::Int(1))),
                    Arc::new(super::types::Expr::Value(Value::Int(2))),
                ])
            ),
            "(1 2)"
        );
    }
}

#[cfg(test)]
mod test_var_storage {
    use super::scope::LexicalVarStorage;
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
