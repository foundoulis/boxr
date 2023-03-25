#[derive(Clone, Debug, PartialEq)]
pub struct ConsCellCreateError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct LexerError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct ParserError(pub &'static str);

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluatorError(pub &'static str);
