#[derive(Debug)]
pub struct ConsCellCreateError(pub &'static str);

#[derive(Debug, PartialEq)]
pub struct LexerError(pub &'static str);
