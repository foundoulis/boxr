use lazy_static::lazy_static;
use regex::Regex;

use crate::{errors::LexerError, parser::ControlToken};

lazy_static! {
    // static ref LEXERS: Vec<(Regex, ControlToken)> = vec![
    //     (Regex::new(r#"(\()                                           "#).unwrap(), ControlToken::LPAREN),  // 1
    //     (Regex::new(r#"(\))                                           "#).unwrap(), ControlToken::RPAREN),  // 2
    //     (Regex::new(r#"(\')                                           "#).unwrap(), ControlToken::QUOTE),  // 3 Control sequence ()'
    //     (Regex::new(r#"((?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+))"#).unwrap(), ControlToken::FLOAT(0.0)),  // 4 Float
    //     (Regex::new(r#"(-?[0-9]+)                                     "#).unwrap(), ControlToken::INTEGER(0)),  // 5 Integer
    //     (Regex::new(r#"(\s)                                           "#).unwrap(), ControlToken::WHITESPACE),  // 6 Whitespace
    //     (Regex::new(r#"("(?:[^"\\]|\\.)*")                            "#).unwrap(), ControlToken::STRING("".to_string())),  // 7 String
    //     (Regex::new(r#"(^[#!].*)                                      "#).unwrap(), ControlToken::COMMENT),  // 8 comments
    //     (Regex::new(r#"(;.*)                                          "#).unwrap(), ControlToken::COMMENT),  // 9 other comments
    //     (Regex::new(r#"([^.\s\'\"\(\);][^\s\'\"\(\);]*)               "#).unwrap(), ControlToken::SYMBOL("".to_string())),  // 10 symbols
    //     (Regex::new(r#"(.*)                                           "#).unwrap(), ControlToken::ERROR),  // 11 Syntax error for anything else.
    // ];
    // static ref LEXER: Regex = Regex::new(r##"(?x)
    //     (\()                                             # 1 (
    //     |(\))                                            # 2 )
    //     |(')                                             # 3 quote
    //     |((?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+)) # 4 float
    //     |(-?[0-9]+)                                      # 5 integer
    //     |(\s)                                            # 6 whitespace
    //     |("(?:[^"\\]|\\.)*")                             # 7 string
    //     |(^[#!].*)                                       # 8 comments
    //     |(;.*)                                           # 9 comments
    //     |([^.\s'"\(\);][^\s'"\(\);]*)                    # 10 symbols
    //     |(.*)                                            # 11 error
    // "##).unwrap();
    static ref LEXER: Regex = Regex::new(r##"(\()|(\))|(')|((?:-?[0-9]+[\.][0-9]*)|(?:-?[0-9]*[\.][0-9]+))|(-?[0-9]+)|(\s)|("(?:[^"\\]|\\.)*")|(^[#!].*)|(;.*)|([^.\s'"\(\);][^\s'"\(\);]*)|(.*)"##).unwrap();
}

pub fn lex(code: String) -> Vec<ControlToken> {
    if code.len() == 0 {
        return vec![];
    }
    LEXER
        .captures_iter(&code)
        .map(|cap| {
            if let Some(_) = cap.get(1) {
                Ok(ControlToken::LPAREN)
            } else if let Some(_) = cap.get(2) {
                Ok(ControlToken::RPAREN)
            } else if let Some(_) = cap.get(3) {
                Ok(ControlToken::QUOTE)
            } else if let Some(f) = cap.get(4) {
                Ok(ControlToken::FLOAT(f.as_str().parse().unwrap()))
            } else if let Some(i) = cap.get(5) {
                Ok(ControlToken::INTEGER(i.as_str().parse().unwrap()))
            } else if let Some(_) = cap.get(6) {
                Ok(ControlToken::WHITESPACE)
            } else if let Some(s) = cap.get(7) {
                Ok(ControlToken::STRING(s.as_str().to_string()))
            } else if let Some(_) = cap.get(8) {
                Ok(ControlToken::COMMENT)
            } else if let Some(_) = cap.get(9) {
                Ok(ControlToken::COMMENT)
            } else if let Some(s) = cap.get(10) {
                Ok(ControlToken::SYMBOL(s.as_str().to_string()))
            } else if let Some(_) = cap.get(11) {
                Err(LexerError("Syntax Error"))
            } else {
                unreachable!()
            }
        })
        .filter_map(|item| match item {
            Ok(ct) => match ct {
                ControlToken::LPAREN
                | ControlToken::RPAREN
                | ControlToken::QUOTE
                | ControlToken::FLOAT(_)
                | ControlToken::INTEGER(_)
                | ControlToken::STRING(_)
                | ControlToken::SYMBOL(_) => Some(ct),
                _ => None,
            },
            Err(err) => Some(ControlToken::ERROR(err)),
        })
        .collect()
}
