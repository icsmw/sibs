mod bindings;
mod parsing;

use crate::*;
pub(super) use test_utils::Files;

pub(super) fn parser(source: &str) -> Parser {
    let mut lexer = Lexer::new(source, 0);
    Parser::unbound(lexer.read().unwrap().tokens, &lexer.uuid, source, false)
}
