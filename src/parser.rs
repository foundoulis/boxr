use crate::errors::LexerError;

#[derive(Debug, PartialEq)]
pub enum ControlToken {
    LPAREN,
    RPAREN,
    QUOTE,
    FLOAT(f64),
    INTEGER(i64),
    STRING(String),
    SYMBOL(String),
    WHITESPACE,
    COMMENT,
    ERROR(LexerError),
}
