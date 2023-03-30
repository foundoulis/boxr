pub mod errors;
pub mod evaluator;
pub mod logger;
pub mod types;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub slyther);

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
    use crate::types::Expr;

    #[test]
    fn test_set_get() {
        let mut storage = LexicalVarStorage::new();
        storage.put("a", Expr::Value(Value::Int(1)));
        assert_eq!(storage.get("a"), Some(&Expr::Value(Value::Int(1))));
    }

    #[test]
    fn test_fork() {
        let mut storage = LexicalVarStorage::new();
        storage.put("a", Expr::Value(Value::Int(1)));
        let mut fork = storage.fork();
        fork.put("b", Expr::Value(Value::Int(2)));
        assert_eq!(storage.get("a"), Some(&Expr::Value(Value::Int(1))));
        assert_eq!(storage.get("b"), None);
        assert_eq!(fork.get("a"), Some(&Expr::Value(Value::Int(1))));
        assert_eq!(fork.get("b"), Some(&Expr::Value(Value::Int(2))));
    }
}

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
        let expr = lisp_eval(&parsed_input[0], &mut LexicalVarStorage::new());
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
        let expr = lisp_eval(&parsed_input[0], &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
mod test_builtin_functions {
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
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
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(0.0)));
    }

    #[test]
    fn test_modulus() {
        let input = "(% 10 3)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(1.0)));
    }

    #[test]
    fn test_pow() {
        let input = "(^ 2 3)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Float(8.0)));
    }

    #[test]
    fn test_eq() {
        let input = "(= 1 1 1 1 1)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Boolean(true)));

        let input = "(= 1 1 1 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Boolean(false)));
    }

    #[test]
    fn test_neq() {
        let input = "(!= 1 1 1 1 1)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Boolean(false)));

        let input = "(!= 1 1 1 1 2)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut LexicalVarStorage::new());
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Expr::Value(Value::Boolean(true)));
    }
}

#[cfg(test)]
mod test_builtin_macro {
    use std::sync::Arc;

    use crate::{
        errors::EvaluatorError,
        evaluator::lisp_eval,
        slyther::ExprsParser,
        types::{scope::LexicalVarStorage, userfunctions::UserDefinedFunction, Expr, Value},
    };

    #[test]
    fn test_define_var() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define a 1)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);
        let expr = &parsed_input[0];
        let result = lisp_eval(expr, &mut storage);
        assert!(result.is_ok());
        assert_eq!(storage.get("a"), Some(&Expr::Value(Value::Int(1))));
    }

    #[test]
    fn test_define_var_access_var() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define a 1) a";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 2);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Vec<_>>();
        assert!(results.iter().all(|result| result.is_ok()));
        let results = results
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(results[1], Expr::Value(Value::Int(1)));
    }

    #[test]
    fn test_define_var_hard_second_arg() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define a (+ 1 2)) a";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 2);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Vec<_>>();
        assert!(results.iter().all(|result| result.is_ok()));
        let results = results
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(results[1], Expr::Value(Value::Float(3.0)));
    }

    #[test]
    fn test_define_var_second_arg_var() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define a 1) (define b (+ a 10)) b";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 3);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Vec<_>>();
        assert!(results.iter().all(|result| result.is_ok()));
        let results = results
            .into_iter()
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(results[1], Expr::Value(Value::NIL));
        assert_eq!(results[2], Expr::Value(Value::Float(11.0)));
    }

    #[test]
    fn test_define_func() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define (func-name x y z) (body))";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Result<Vec<Expr>, EvaluatorError>>();
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(
            storage.get_func("func-name"),
            Some(&UserDefinedFunction::new(
                vec![
                    Expr::Value(Value::Symbol("x".to_string())),
                    Expr::Value(Value::Symbol("y".to_string())),
                    Expr::Value(Value::Symbol("z".to_string()))
                ],
                vec![Expr::List(vec![Arc::new(Expr::Value(Value::Symbol(
                    "body".to_string()
                )))])]
            ))
        );
    }

    #[test]
    fn test_define_func_and_call() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define (func-name x y z) (+ x y z))";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Result<Vec<Expr>, EvaluatorError>>();
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(
            storage.get_func("func-name"),
            Some(&UserDefinedFunction::new(
                vec![
                    Expr::Value(Value::Symbol("x".to_string())),
                    Expr::Value(Value::Symbol("y".to_string())),
                    Expr::Value(Value::Symbol("z".to_string()))
                ],
                vec![Expr::List(vec![
                    Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("x".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("y".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("z".to_string())))
                ])]
            ))
        );

        let input = "(func-name 1 2 3)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 1);

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Result<Vec<Expr>, EvaluatorError>>();
        println!("{:?}", results);
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], Expr::Value(Value::Float(6.0)));
    }

    #[test]
    fn test_define_func_and_call_and_return() {
        let mut storage = LexicalVarStorage::new();
        let input = "(define (func-name x y z) (print x) (+ x y z)) (func-name 1 2 3)";
        let parsed_input = ExprsParser::new().parse(input);
        assert!(parsed_input.is_ok());
        let parsed_input = parsed_input.unwrap();
        assert_eq!(parsed_input.len(), 2);
        assert_eq!(
            parsed_input[0],
            Arc::new(Expr::List(vec![
                Arc::new(Expr::Value(Value::Symbol("define".to_string()))),
                Arc::new(Expr::List(vec![
                    Arc::new(Expr::Value(Value::Symbol("func-name".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("x".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("y".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("z".to_string())))
                ])),
                Arc::new(Expr::List(vec![
                    Arc::new(Expr::Value(Value::Symbol("print".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("x".to_string())))
                ])),
                Arc::new(Expr::List(vec![
                    Arc::new(Expr::Value(Value::Symbol("+".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("x".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("y".to_string()))),
                    Arc::new(Expr::Value(Value::Symbol("z".to_string())))
                ]))
            ]))
        );
        assert_eq!(
            parsed_input[1],
            Arc::new(Expr::List(vec![
                Arc::new(Expr::Value(Value::Symbol("func-name".to_string()))),
                Arc::new(Expr::Value(Value::Int(1))),
                Arc::new(Expr::Value(Value::Int(2))),
                Arc::new(Expr::Value(Value::Int(3)))
            ]))
        );

        let results = parsed_input
            .iter()
            .map(|expr| lisp_eval(expr, &mut storage))
            .collect::<Result<Vec<Expr>, EvaluatorError>>();
        println!("{:?}", results);
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], Expr::Value(Value::NIL));
        assert_eq!(results[1], Expr::Value(Value::Float(6.0)));
    }
}
