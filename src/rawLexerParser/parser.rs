use crate::{
    errors::{LexerError, ParserError},
    types::BoxrType,
};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum StackElem<'s> {
    BoxrType(BoxrType<'s>),
    ControlToken(ControlToken),
}

impl<'s> From<BoxrType<'s>> for StackElem<'s> {
    fn from(item: BoxrType<'s>) -> Self {
        StackElem::BoxrType(item)
    }
}

impl From<ControlToken> for StackElem<'_> {
    fn from(item: ControlToken) -> Self {
        StackElem::ControlToken(item)
    }
}

impl<'s> TryFrom<StackElem<'s>> for BoxrType<'s> {
    type Error = ParserError;

    fn try_from(value: StackElem<'s>) -> Result<Self, Self::Error> {
        match value {
            StackElem::BoxrType(bt) => Ok(bt),
            _ => Err(ParserError("Not a BoxrType")),
        }
    }
}

impl TryFrom<StackElem<'_>> for ControlToken {
    type Error = ParserError;

    fn try_from(value: StackElem) -> Result<Self, Self::Error> {
        match value {
            StackElem::ControlToken(ct) => Ok(ct),
            _ => Err(ParserError("Not a ControlToken")),
        }
    }
}

pub fn parse<'s>(tokens: Vec<ControlToken>) -> Result<Vec<StackElem<'s>>, ParserError> {
    let token_copy = tokens
        .clone()
        .into_iter()
        .map(|ct| StackElem::from(ct))
        .collect::<Vec<StackElem>>();

    let mut stack: Vec<StackElem> = Vec::new();
    for elem in token_copy {
        let mut was_lparen = false;

        if elem == StackElem::from(ControlToken::RPAREN) {
            let mut start = StackElem::from(BoxrType::NIL);

            for x in stack.clone().into_iter().rev() {
                let check = x;
                if check == StackElem::from(ControlToken::LPAREN) {
                    was_lparen = true;
                    stack.pop();
                    stack.push(start);
                    break;
                } else if match check {
                    StackElem::BoxrType(BoxrType::CELL(_, _)) => true,
                    StackElem::ControlToken(ControlToken::QUOTE) => true,
                    StackElem::ControlToken(ControlToken::STRING(_)) => true,
                    StackElem::ControlToken(ControlToken::INTEGER(_)) => true,
                    StackElem::ControlToken(ControlToken::FLOAT(_)) => true,
                    StackElem::ControlToken(ControlToken::SYMBOL(_)) => true,
                    StackElem::BoxrType(BoxrType::NIL) => true,
                    _ => false,
                } {
                    start = StackElem::from(BoxrType::cons_cell(
                        BoxrType::try_from(check.clone()).unwrap(),
                        BoxrType::try_from(start).unwrap(),
                    ));
                    stack.pop();
                }
                // else if check == StackElem::from(BoxrType::QUOTE) {
                //     return Err(ParserError("Invalid quote"));
                // }
            }
            if !was_lparen {
                return Err(ParserError("Unmatched RPAREN"));
            }
        } else {
            stack.push(elem.clone());
            if let &StackElem::ControlToken(_) = &elem {
                continue;
            }
        }
    }
    Ok(stack)
}
