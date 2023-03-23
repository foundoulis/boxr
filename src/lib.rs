pub mod errors;
pub mod lexer;
pub mod parser;
pub mod types;

#[cfg(test)]
mod types_tests {
    use crate::types::BoxrType;

    #[test]
    fn test_boxr_non_cons() {
        assert!(BoxrType::NIL == BoxrType::nil());
    }

    #[test]
    fn test_bool_type() {
        let t = BoxrType::BOOL(true);
        assert!(t.bool());
        let f = BoxrType::BOOL(false);
        assert!(!f.bool());
        assert!(t.bool() != f.bool());
        assert!(!t.bool() == f.bool());

        let t_from_raw = BoxrType::from(true);
        let f_from_raw = BoxrType::from(false);
        assert!(t.bool() == t_from_raw.bool());
        assert!(f.bool() == f_from_raw.bool());

        assert!(!BoxrType::INT(0).bool());
        assert!(BoxrType::INT(420).bool());

        assert!(!BoxrType::STR("").bool());
        assert!(BoxrType::STR("not empty").bool());
    }

    #[test]
    fn test_int_type() {
        let j = 69;
        let from_j = BoxrType::from(j);
        assert!(from_j == BoxrType::INT(j));

        let i = BoxrType::INT(420);
        let into_i = i.clone() as BoxrType;
        assert!(into_i == i);
    }

    #[test]
    fn test_str_type() {
        let s = "Some string";
        let from_s = BoxrType::from(s);
        assert!(from_s == BoxrType::from(s));

        let t = BoxrType::STR("Some string");
        let into_t = t.clone() as BoxrType;
        assert!(into_t == t);
    }

    #[test]
    fn test_cons_cell() {
        let a = BoxrType::cons_cell(BoxrType::INT(5), BoxrType::INT(3));
        assert!(a.decompose() == Some((BoxrType::INT(5), BoxrType::INT(3))));
        let b = BoxrType::cons_cell(BoxrType::INT(3), BoxrType::INT(5));
        let (b_car, b_cdr) = b.decompose().unwrap();
        assert!(b_car == BoxrType::INT(3));
        assert!(b_cdr == BoxrType::INT(5));
        let c = BoxrType::cons_cell(BoxrType::STR("car"), BoxrType::NIL);
        assert!(c.cdr().unwrap() == BoxrType::NIL);
    }

    #[test]
    fn test_cons_cell_debug() {
        let cell = BoxrType::cons_cell(BoxrType::NIL, BoxrType::INT(10));
        assert_eq!("(cons NIL 10)", format!("{:?}", cell));
        let two_cells = BoxrType::cons_cell(
            BoxrType::cons_cell(BoxrType::INT(2), BoxrType::INT(1)),
            BoxrType::INT(1),
        );
        assert_eq!("(cons (cons 2 1) 1)", format!("{:?}", two_cells));
    }

    #[test]
    fn test_cons_cell_display() {
        let cell: BoxrType = vec![BoxrType::NIL, BoxrType::INT(10)].into_iter().collect();
        assert_eq!("('() 10)", format!("{}", cell));
        let two_cells: BoxrType = vec![
            vec![BoxrType::INT(2), BoxrType::INT(1)]
                .into_iter()
                .collect::<BoxrType>(),
            BoxrType::INT(1),
        ]
        .into_iter()
        .collect();
        assert_eq!("((2 1) 1)", format!("{}", two_cells));
    }

    #[test]
    fn test_cons_list_create() {
        let cell = BoxrType::cons_cell(BoxrType::INT(1), BoxrType::NIL);
        let (cell_car, cell_cdr) = cell.decompose().unwrap();
        assert!(cell_car == BoxrType::INT(1));
        assert!(cell_cdr == BoxrType::NIL);
    }

    #[test]
    fn test_cons_list_from_iter() {
        let values = vec![
            BoxrType::INT(1),
            BoxrType::STR("Second elem"),
            BoxrType::cons_cell(BoxrType::INT(0), BoxrType::INT(2)),
        ];
        let cons_list: BoxrType = values.into_iter().collect();
        let (mut head, mut body) = cons_list.decompose().unwrap();
        assert!(head == BoxrType::INT(1));
        (head, body) = body.decompose().unwrap();
        assert!(head == BoxrType::STR("Second elem"));
        (head, body) = body.decompose().unwrap();
        assert!(head == BoxrType::CELL(Box::new(BoxrType::INT(0)), Box::new(BoxrType::INT(2))));
        assert!(body == BoxrType::NIL);
    }

    #[test]
    fn test_cons_list_to_iter() {
        let cons_list = vec![
            BoxrType::INT(1),
            BoxrType::STR("Second elem"),
            BoxrType::cons_cell(BoxrType::INT(0), BoxrType::INT(2)),
        ]
        .into_iter()
        .collect::<BoxrType>();
        let mut cons_list_iter = cons_list.into_iter();
        assert!(cons_list_iter.next() == Some(BoxrType::INT(1)));
        assert!(cons_list_iter.next() == Some(BoxrType::STR("Second elem")));
        assert!(
            cons_list_iter.next()
                == Some(BoxrType::CELL(
                    Box::new(BoxrType::INT(0)),
                    Box::new(BoxrType::INT(2))
                ))
        );
        assert!(cons_list_iter.next() == None);
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::lex;
    use crate::parser::ControlToken;

    #[test]
    fn test_lexer_empty() {
        let lexer_raw = lex("".to_string());
        println!("{:?}", lexer_raw);
        let mut lexer = lexer_raw.iter();
        assert!(lexer.next() == None);
        assert!(lexer_raw.len() == 0);
    }

    #[test]
    fn test_lexer_simple() {
        let lexer_raw = lex("(+ 1 2)".to_string());
        println!("{:?}", lexer_raw);
        let mut lexer = lexer_raw.iter();
        assert!(lexer.next() == Some(&ControlToken::LPAREN));
        assert!(lexer.next() == Some(&ControlToken::SYMBOL("+".to_string())));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(1)));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(2)));
        assert!(lexer.next() == Some(&ControlToken::RPAREN));
        assert!(lexer.next() == None);
    }

    #[test]
    fn test_lexer_with_whitespace() {
        let lexer_raw = lex("  (+ 1 2)  ".to_string());

        println!("{:?}", lexer_raw);
        let mut lexer = lexer_raw.iter();
        assert!(lexer.next() == Some(&ControlToken::LPAREN));
        assert!(lexer.next() == Some(&ControlToken::SYMBOL("+".to_string())));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(1)));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(2)));
        assert!(lexer.next() == Some(&ControlToken::RPAREN));
        assert!(lexer.next() == None);
    }

    #[test]
    fn test_lexer_with_comments() {
        let lexer_raw = lex("  (+ 1 2) ; comment".to_string());
        println!("{:?}", lexer_raw);
        let mut lexer = lexer_raw.iter();
        assert!(lexer.next() == Some(&ControlToken::LPAREN));
        assert!(lexer.next() == Some(&ControlToken::SYMBOL("+".to_string())));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(1)));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(2)));
        assert!(lexer.next() == Some(&ControlToken::RPAREN));
        assert!(lexer.next() == None);
    }

    #[test]
    fn test_lexer_with_comments_and_whitespace() {
        let lexer_raw = lex(" (+ 1 2) ; comment

        "
        .to_string());
        println!("{:?}", lexer_raw);
        let mut lexer = lexer_raw.iter();
        assert!(lexer.next() == Some(&ControlToken::LPAREN));
        assert!(lexer.next() == Some(&ControlToken::SYMBOL("+".to_string())));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(1)));
        assert!(lexer.next() == Some(&ControlToken::INTEGER(2)));
        assert!(lexer.next() == Some(&ControlToken::RPAREN));
    }
}
