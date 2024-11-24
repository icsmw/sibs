#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;
use lexer::KindId;

impl ReadNode<ComparisonGroup> for ComparisonGroup {
    fn read(parser: &mut Parser) -> Result<Option<ComparisonGroup>, LinkedErr<E>> {
        let Some((mut inner, ..)) = parser.between(KindId::LeftParen, KindId::RightParen)? else {
            return Ok(None);
        };
        let mut collected = Vec::new();
        while let Some(node) = Expression::try_oneof(
            &mut inner,
            &[
                ExpressionId::Comparison,
                ExpressionId::LogicalOp,
                ExpressionId::ComparisonGroup,
            ],
        )?
        .map(Node::Expression)
        {
            if let Node::Expression(Expression::LogicalOp(op)) = &node {
                match collected.last() {
                    Some(Node::Expression(Expression::LogicalOp(prev))) => {
                        return Err(E::UnexpectedLogicalOperator(prev.token.id())
                            .link_with_token(&prev.token));
                    }
                    None => {
                        return Err(
                            E::UnexpectedLogicalOperator(op.token.id()).link_with_token(&op.token)
                        );
                    }
                    Some(_) => {}
                }
            } else {
                match collected.last() {
                    Some(Node::Expression(Expression::LogicalOp(..))) | None => {}
                    Some(n) => {
                        return Err(E::MissedLogicalOperator.link(&n.into()));
                    }
                }
            }
            collected.push(node);
        }
        Ok(if collected.is_empty() || !inner.is_done() {
            None
        } else {
            Some(ComparisonGroup {
                nodes: collected,
                uuid: Uuid::new_v4(),
            })
        })
    }
}
