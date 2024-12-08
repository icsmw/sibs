#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ComparisonSeq {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::LeftParen
                | Kind::Identifier(..)
                | Kind::Number(..)
                | Kind::String(..)
                | Kind::InterpolatedString(..)
                | Kind::Keyword(Keyword::True)
                | Kind::Keyword(Keyword::False)
        )
    }
}

impl ReadNode<ComparisonSeq> for ComparisonSeq {
    fn read(parser: &mut Parser) -> Result<Option<ComparisonSeq>, LinkedErr<E>> {
        let mut collected: Vec<LinkedNode> = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Expression(&[
                ExpressionId::Variable,
                ExpressionId::Comparison,
                ExpressionId::LogicalOp,
                ExpressionId::ComparisonGroup,
            ])],
        )? {
            if let Node::Expression(Expression::LogicalOp(op)) = &(node.node) {
                match collected.last().map(|n| &n.node) {
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
                match collected.last().map(|n| &n.node) {
                    Some(Node::Expression(Expression::LogicalOp(..))) | None => {}
                    Some(Node::Expression(Expression::Variable(..))) => {
                        if collected.len() == 1 {
                            return Ok(None);
                        } else {
                            return Err(E::MissedLogicalOperator.link(&(&node).into()));
                        }
                    }
                    Some(n) => {
                        return Err(E::MissedLogicalOperator.link(&n.into()));
                    }
                }
            }
            collected.push(node);
        }
        Ok(if collected.is_empty() {
            None
        } else {
            Some(ComparisonSeq {
                nodes: collected,
                uuid: Uuid::new_v4(),
            })
        })
    }
}
