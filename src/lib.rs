pub mod errors;
pub mod evaluator;
pub mod logger;
pub mod types;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub slyther);

#[cfg(test)]
mod test_atoms_ops {
    use std::sync::Arc;

    #[test]
    fn test_nil() {
        let nil = super::types::Cons::Value(super::types::ConsValue::NIL);
        assert_eq!(nil.is_nil(), true);
        assert_eq!(nil.is_quoted(), false);
    }

    #[test]
    fn test_nil_false() {
        let nil = super::types::Cons::Value(super::types::ConsValue::Int(123));
        assert_eq!(nil.is_nil(), false);
        assert_eq!(nil.is_quoted(), false);
    }

    #[test]
    fn test_is_quoted() {
        let nil = super::types::Cons::Quoted(Arc::new(super::types::Cons::Value(
            super::types::ConsValue::NIL,
        )));
        assert_eq!(nil.is_nil(), false);
        assert_eq!(nil.is_quoted(), true);
    }

    #[test]
    fn test_is_quoted_false() {
        let nil = super::types::Cons::Value(super::types::ConsValue::Int(123));
        assert_eq!(nil.is_nil(), false);
        assert_eq!(nil.is_quoted(), false);
    }

    #[test]
    fn test_is_nil_value() {
        let nil = super::types::ConsValue::NIL;
        assert_eq!(nil.is_nil(), true);
    }

    #[test]
    fn test_is_not_nil_value() {
        let nil = super::types::ConsValue::Int(123);
        assert_eq!(nil.is_nil(), false);
    }
}

#[cfg(test)]
mod test_parse_atom {

    #[test]
    fn test_nil() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("'()");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::NIL)
        );
    }

    #[test]
    fn test_int() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("123");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::Int(123))
        );
    }

    #[test]
    fn test_float() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("123.456");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::Float(123.456))
        );
    }

    #[test]
    fn test_string() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("\"123\"");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::String("123".to_string()))
        );
    }

    #[test]
    fn test_symbol() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("abc");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::Symbol("abc".to_string()))
        );
    }

    #[test]
    fn test_comment() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse(";abc");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::Comment("abc".to_string()))
        );
    }
}

#[cfg(test)]
mod test_parse_cons {
    use std::sync::Arc;

    #[test]
    fn test_nil_list() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("'()");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::NIL)
        );
    }

    #[test]
    fn test_value_cons() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("(123)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Value(super::types::ConsValue::Int(123))
        );
    }

    #[test]
    fn test_list_many() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("(123 456)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Cell(
                Arc::new(super::types::Cons::Value(super::types::ConsValue::Int(123))),
                Arc::new(super::types::Cons::Cell(
                    Arc::new(super::types::Cons::Value(super::types::ConsValue::Int(456))),
                    Arc::new(super::types::Cons::Value(super::types::ConsValue::NIL))
                ))
            )
        );
    }

    #[test]
    fn test_quoted_list_single() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("'(123)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Quoted(Arc::new(super::types::Cons::Value(
                super::types::ConsValue::Int(123)
            )))
        );
    }

    #[test]
    fn test_quoted_list_double() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("''(123)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Quoted(Arc::new(super::types::Cons::Quoted(Arc::new(
                super::types::Cons::Value(super::types::ConsValue::Int(123))
            ))))
        );
    }

    #[test]
    fn test_quoted_list_single_many_items() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("'(123 456)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::Quoted(Arc::new(super::types::Cons::Cell(
                Arc::new(super::types::Cons::Value(super::types::ConsValue::Int(123))),
                Arc::new(super::types::Cons::Cell(
                    Arc::new(super::types::Cons::Value(super::types::ConsValue::Int(456))),
                    Arc::new(super::types::Cons::Value(super::types::ConsValue::NIL))
                ))
            )))
        );
    }

    #[test]
    fn test_from_iter_single() {
        assert_eq!(
            &super::types::Cons::Cell(
                Arc::new(super::types::Cons::Value(super::types::ConsValue::Int(123))),
                Arc::new(super::types::Cons::Value(super::types::ConsValue::NIL))
            ),
            &super::types::Cons::from_iter(vec![super::types::Cons::Value(
                super::types::ConsValue::Int(123)
            )])
        );
    }

    #[test]
    fn test_from_iter_many() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("(123 456 789)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(
            exprs[0].as_ref(),
            &super::types::Cons::from_iter(vec![
                super::types::Cons::Value(super::types::ConsValue::Int(123)),
                super::types::Cons::Value(super::types::ConsValue::Int(456)),
                super::types::Cons::Value(super::types::ConsValue::Int(789)),
            ])
        );
    }

    #[test]
    fn test_from_iter_loop() {
        let parser = super::slyther::SExpressionsParser::new();
        let exprs = parser.parse("(123 456 789)");
        assert!(exprs.is_ok());
        let exprs = exprs.unwrap();
        assert_eq!(exprs.len(), 1);
        let mut iter = super::types::Cons::clone(&exprs[0]).into_iter();
        assert_eq!(
            iter.next(),
            Some(super::types::Cons::Value(super::types::ConsValue::Int(123)))
        );
        assert_eq!(
            iter.next(),
            Some(super::types::Cons::Value(super::types::ConsValue::Int(456)))
        );
        assert_eq!(
            iter.next(),
            Some(super::types::Cons::Value(super::types::ConsValue::Int(789)))
        );
        assert_eq!(iter.next(), None);
    }
}

#[cfg(test)]
mod test_lexvar_stg {
    use crate::types::scope::LexicalVarStorage;
    use crate::types::Cons;
    use crate::types::ConsValue;

    #[test]
    fn test_lexvar_stg() {
        let mut stg = LexicalVarStorage::new();
        assert_eq!(stg.get("foo"), None);
        assert_eq!(stg.get("bar"), None);
        stg.put("foo", Cons::Value(ConsValue::Int(123)));
        assert_eq!(stg.get("foo"), Some(&Cons::Value(ConsValue::Int(123))));
        assert_eq!(stg.get("bar"), None);
        stg.put("bar", Cons::Value(ConsValue::Int(456)));
        assert_eq!(stg.get("foo"), Some(&Cons::Value(ConsValue::Int(123))));
        assert_eq!(stg.get("bar"), Some(&Cons::Value(ConsValue::Int(456))));
    }

    #[test]
    fn test_lexvar_frk() {
        let mut stg = LexicalVarStorage::new();
        stg.put("foo", Cons::Value(ConsValue::Int(123)));
        stg.put("bar", Cons::Value(ConsValue::Int(456)));
        let mut stg2 = stg.fork();
        assert_eq!(stg2.get("foo"), Some(&Cons::Value(ConsValue::Int(123))));
        assert_eq!(stg2.get("bar"), Some(&Cons::Value(ConsValue::Int(456))));
        stg2.put("foo", Cons::Value(ConsValue::Int(789)));
        assert_eq!(stg2.get("foo"), Some(&Cons::Value(ConsValue::Int(789))));
        assert_eq!(stg2.get("bar"), Some(&Cons::Value(ConsValue::Int(456))));
        assert_eq!(stg.get("foo"), Some(&Cons::Value(ConsValue::Int(123))));
        assert_eq!(stg.get("bar"), Some(&Cons::Value(ConsValue::Int(456))));
    }
}

#[cfg(test)]
mod test_eval_atoms {
    use crate::{evaluator::lisp_eval, types::scope::LexicalVarStorage};
    use std::sync::Arc;

    #[test]
    fn test_eval_int() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Value(crate::types::ConsValue::Int(123));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expr);
    }

    #[test]
    fn test_eval_str() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Value(crate::types::ConsValue::String("foo".to_string()));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expr);
    }

    #[test]
    fn test_eval_float() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Value(crate::types::ConsValue::Float(123.456));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expr);
    }

    #[test]
    fn test_eval_bool() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Value(crate::types::ConsValue::Boolean(true));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expr);
    }

    #[test]
    fn test_single_quote() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Quoted(Arc::new(crate::types::Cons::Value(
            crate::types::ConsValue::Int(123),
        )));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            crate::types::Cons::Value(crate::types::ConsValue::Int(123))
        );
    }

    #[test]
    fn test_multi_quote() {
        let mut stg = LexicalVarStorage::new();
        let expr = crate::types::Cons::Quoted(Arc::new(crate::types::Cons::Quoted(Arc::new(
            crate::types::Cons::Value(crate::types::ConsValue::Int(123)),
        ))));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            crate::types::Cons::Quoted(Arc::new(crate::types::Cons::Value(
                crate::types::ConsValue::Int(123)
            )))
        );
    }
}

#[cfg(test)]
mod test_eval_lists {
    use crate::evaluator::lisp_eval;
    use std::sync::Arc;

    #[test]
    fn test_single_quote_list() {
        let mut stg = crate::types::scope::LexicalVarStorage::new();
        let expr = crate::types::Cons::Quoted(Arc::new(crate::types::Cons::from_iter(vec![
            crate::types::Cons::Value(crate::types::ConsValue::Int(123)),
            crate::types::Cons::Value(crate::types::ConsValue::Int(456)),
            crate::types::Cons::Value(crate::types::ConsValue::Int(789)),
        ])));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            crate::types::Cons::from_iter(vec![
                crate::types::Cons::Value(crate::types::ConsValue::Int(123)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(456)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(789)),
            ])
        );
    }

    #[test]
    fn test_multi_quote_list() {
        let mut stg = crate::types::scope::LexicalVarStorage::new();
        let expr = crate::types::Cons::Quoted(Arc::new(crate::types::Cons::Quoted(Arc::new(
            crate::types::Cons::from_iter(vec![
                crate::types::Cons::Value(crate::types::ConsValue::Int(123)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(456)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(789)),
            ]),
        ))));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            crate::types::Cons::Quoted(Arc::new(crate::types::Cons::from_iter(vec![
                crate::types::Cons::Value(crate::types::ConsValue::Int(123)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(456)),
                crate::types::Cons::Value(crate::types::ConsValue::Int(789)),
            ])))
        );
    }
}

#[cfg(test)]
mod test_func_built {
    use crate::{
        evaluator::lisp_eval,
        types::{scope::LexicalVarStorage, Cons, ConsValue},
    };

    #[test]
    fn test_func_add() {
        let mut stg = crate::types::scope::LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("+".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(579)));
    }

    #[test]
    fn test_func_add_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("+".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(0)));
    }

    #[test]
    fn test_func_add_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("+".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(123)));
    }

    #[test]
    fn test_func_sub() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("-".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(-333)));
    }

    #[test]
    fn test_func_sub_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("-".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(0)));
    }

    #[test]
    fn test_func_sub_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("-".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(-123)));
    }

    #[test]
    fn test_func_mult() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("*".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(56088)));
    }

    #[test]
    fn test_func_mult_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("*".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(1)));
    }

    #[test]
    fn test_func_mult_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("*".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(123)));
    }

    #[test]
    fn test_func_div() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("/".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Float(0.26973684210526316)));
    }

    #[test]
    fn test_func_div_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("/".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(1)));
    }

    #[test]
    fn test_func_div_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("/".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Float(0.008130081300813009)));
    }

    #[test]
    fn test_func_floordiv() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("floordiv".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(0)));
    }

    #[test]
    fn test_func_mod() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("%".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(123)));
    }

    #[test]
    fn test_func_pow() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("^".to_string())),
            Cons::Value(ConsValue::Int(2)),
            Cons::Value(ConsValue::Int(2)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(4)));
    }

    #[test]
    fn test_func_pow_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("^".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(1)));
    }

    #[test]
    fn test_func_pow_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("^".to_string())),
            Cons::Value(ConsValue::Int(2)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(2)));
    }

    #[test]
    fn test_func_eq() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));
    }

    #[test]
    fn test_func_eq_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("=".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));
    }

    #[test]
    fn test_func_eq_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("=".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));
    }

    #[test]
    fn test_func_neq() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("!=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("!=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));
    }

    #[test]
    fn test_func_neq_no_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![Cons::Value(ConsValue::Symbol("!=".to_string()))]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));
    }

    #[test]
    fn test_func_neq_one_arg() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("!=".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));
    }

    #[test]
    fn test_func_lt() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<".to_string())),
            Cons::Value(ConsValue::Int(456)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));
    }

    #[test]
    fn test_func_gt() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">".to_string())),
            Cons::Value(ConsValue::Int(456)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));
    }

    #[test]
    fn test_func_lte() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("<=".to_string())),
            Cons::Value(ConsValue::Int(456)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));
    }

    #[test]
    fn test_func_gte() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(false)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">=".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol(">=".to_string())),
            Cons::Value(ConsValue::Int(456)),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Boolean(true)));
    }
}

#[cfg(test)]
mod test_mcro_built {
    use crate::{
        evaluator::lisp_eval,
        types::{scope::LexicalVarStorage, Cons, ConsValue},
    };

    #[test]
    fn test_set_define_var_atom() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::Value(ConsValue::Symbol("a".to_string())),
            Cons::Value(ConsValue::Int(123)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        let expr = Cons::Value(ConsValue::Symbol("a".to_string()));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(123)));
    }

    #[test]
    fn test_set_define_var_eval() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::Value(ConsValue::Symbol("a".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("+".to_string())),
                Cons::Value(ConsValue::Int(123)),
                Cons::Value(ConsValue::Int(456)),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        let expr = Cons::Value(ConsValue::Symbol("a".to_string()));
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(579)));
    }

    #[test]
    fn test_define_real_func() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("add".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("+".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("add".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(579)));
    }

    #[test]
    fn test_define_empty_func() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("add".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("add".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));
    }

    #[test]
    fn test_define_multi_line_func() {
        let mut stg = LexicalVarStorage::new();
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("add-then-sub".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("+".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("-".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("add-then-sub".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(-333)));
    }

    #[test]
    fn test_function_in_function() {
        let mut stg = LexicalVarStorage::new();

        // Create a function that overrides the + function
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("add".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("+".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        // Invoke the new add function
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("add".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(579)));

        // Create a function that uses the new add function
        // As part of its definition
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("define".to_string())),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("addadd".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
            ]),
            Cons::from_iter(vec![
                Cons::Value(ConsValue::Symbol("add".to_string())),
                Cons::Value(ConsValue::Symbol("a".to_string())),
                Cons::Value(ConsValue::Symbol("b".to_string())),
                Cons::from_iter(vec![
                    Cons::Value(ConsValue::Symbol("add".to_string())),
                    Cons::Value(ConsValue::Symbol("a".to_string())),
                    Cons::Value(ConsValue::Symbol("b".to_string())),
                ]),
            ]),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::NIL));

        // Invoke the new addadd function
        let expr = Cons::from_iter(vec![
            Cons::Value(ConsValue::Symbol("addadd".to_string())),
            Cons::Value(ConsValue::Int(123)),
            Cons::Value(ConsValue::Int(456)),
        ]);
        let result = lisp_eval(&expr, &mut stg);
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, Cons::Value(ConsValue::Int(579)));
    }
}
