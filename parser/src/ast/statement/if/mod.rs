#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::{Keyword, Kind};

impl ReadNode<If> for If {
    fn read(parser: &mut Parser) -> Result<Option<If>, LinkedErr<E>> {
        let mut cases = Vec::new();
        loop {
            let restore = parser.pin();
            if let Some(tk) = parser.token().cloned() {
                match tk.kind {
                    Kind::Keyword(Keyword::If) => {
                        let Some(cond) = Expression::try_read(parser, ExpressionId::ComparisonSeq)?
                            .map(Node::Expression)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                ExpressionId::ComparisonSeq.to_string(),
                            )
                            .link_with_token(&tk));
                        };
                        let Some(blk) =
                            Statement::try_read(parser, StatementId::Block)?.map(Node::Statement)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            )
                            .link_with_token(&tk));
                        };
                        cases.push(IfCase::If(cond, blk, tk));
                    }
                    Kind::Keyword(Keyword::Else) => {
                        let Some(blk) =
                            Statement::try_read(parser, StatementId::Block)?.map(Node::Statement)
                        else {
                            return Err(E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            )
                            .link_with_token(&tk));
                        };
                        cases.push(IfCase::Else(blk, tk));
                    }
                    _ => {
                        restore(parser);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        if cases.is_empty() {
            Ok(None)
        } else {
            Ok(Some(If { cases }))
        }
    }
}
