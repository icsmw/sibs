#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use diagnostics::*;

impl ReadNode<BinaryExpSeq> for BinaryExpSeq {
    fn read(parser: &mut Parser) -> Result<Option<BinaryExpSeq>, LinkedErr<E>> {
        let mut collected: Vec<LinkedNode> = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[
                NodeReadTarget::Value(&[ValueId::Number]),
                NodeReadTarget::Expression(&[
                    ExpressionId::Variable,
                    ExpressionId::BinaryExp,
                    ExpressionId::BinaryOp,
                    ExpressionId::BinaryExpGroup,
                ]),
            ],
        )? {
            if let Node::Expression(Expression::BinaryOp(op)) = &node.node {
                match collected.last().map(|n| &n.node) {
                    Some(Node::Expression(Expression::BinaryOp(prev))) => {
                        return Err(E::UnexpectedBinaryOperator(prev.token.id())
                            .link_with_token(&prev.token));
                    }
                    None => {
                        return Err(
                            E::UnexpectedBinaryOperator(op.token.id()).link_with_token(&op.token)
                        );
                    }
                    Some(_) => {}
                }
            } else {
                match collected.last().map(|n| &n.node) {
                    Some(Node::Expression(Expression::BinaryOp(..))) | None => {}
                    Some(Node::Expression(Expression::Variable(..))) => {
                        if collected.len() == 1 {
                            return Ok(None);
                        } else {
                            return Err(E::MissedBinaryOperator.link(&(&node).into()));
                        }
                    }
                    Some(n) => {
                        return Err(E::MissedBinaryOperator.link(&n.into()));
                    }
                }
            }
            collected.push(node);
        }
        Ok(if collected.is_empty() {
            None
        } else {
            Some(BinaryExpSeq {
                nodes: collected,
                uuid: Uuid::new_v4(),
            })
        })
    }
}
