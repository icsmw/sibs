#[cfg(test)]
mod proptests;

use crate::*;

impl ReadNode<If> for If {
    fn read(parser: &mut Parser) -> Result<Option<If>, LinkedErr<E>> {
        let mut cases = Vec::new();
        loop {
            let restore = parser.pin();
            if let Some(tk) = parser.token().cloned() {
                match tk.kind {
                    Kind::Keyword(Keyword::If) => {
                        let cond = LinkedNode::try_oneof(
                            parser,
                            &[NodeReadTarget::Expression(&[ExpressionId::ComparisonSeq])],
                        )?
                        .ok_or_else(|| {
                            E::MissedExpectation(
                                tk.id().to_string(),
                                ExpressionId::ComparisonSeq.to_string(),
                            )
                            .link_with_token(&tk)
                        })?;
                        let blk = LinkedNode::try_oneof(
                            parser,
                            &[NodeReadTarget::Statement(&[StatementId::Block])],
                        )?
                        .ok_or_else(|| {
                            E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            )
                            .link_with_token(&tk)
                        })?;
                        cases.push(IfCase::If(cond, blk, tk));
                    }
                    Kind::Keyword(Keyword::Else) => {
                        let blk = LinkedNode::try_oneof(
                            parser,
                            &[NodeReadTarget::Statement(&[StatementId::Block])],
                        )?
                        .ok_or_else(|| {
                            E::MissedExpectation(
                                tk.id().to_string(),
                                StatementId::Block.to_string(),
                            )
                            .link_with_token(&tk)
                        })?;
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
            Ok(Some(If {
                cases,
                uuid: Uuid::new_v4(),
            }))
        }
    }
}
