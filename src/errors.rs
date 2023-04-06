#[derive(Clone, Debug, PartialEq)]
pub struct ConsCellCreateError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct LexerError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct ParserError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub enum EvaluatorError {
    NotAFunction(String),
    UndefinedSymbol(String),
    UncallableType(String),
    BadFunctionDefinition(String),
    InvalidArgument(String),
    ReturnedNonCons(String),
}
