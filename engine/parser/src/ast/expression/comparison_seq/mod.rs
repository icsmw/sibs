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
                | Kind::SingleQuote
                | Kind::Keyword(Keyword::True)
                | Kind::Keyword(Keyword::False)
                | Kind::Bang
        )
    }
}

impl ReadNode<ComparisonSeq> for ComparisonSeq {
    fn read(parser: &Parser) -> Result<Option<ComparisonSeq>, LinkedErr<E>> {
        let mut collected: Vec<LinkedNode> = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[
                ExpressionId::Variable,
                ExpressionId::FunctionCall,
                ExpressionId::Comparison,
                ExpressionId::LogicalOp,
                ExpressionId::ComparisonGroup,
            ])],
        )? {
            if let Node::Expression(Expression::LogicalOp(op)) = &(node.get_node()) {
                match collected.last().map(|n| n.get_node()) {
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
                match collected.last().map(|n| n.get_node()) {
                    Some(Node::Expression(Expression::LogicalOp(..))) | None => {}
                    Some(Node::Expression(Expression::Variable(..))) => {
                        if collected.len() == 1 {
                            return Ok(None);
                        } else {
                            return Err(E::MissedLogicalOperator.link(&node));
                        }
                    }
                    Some(_) => {
                        return Err(E::MissedLogicalOperator.link(&node));
                    }
                }
            }
            collected.push(node);
        }
        if let Some(node) = collected.last() {
            if matches!(node.get_node(), Node::Expression(Expression::LogicalOp(..))) {
                return Err(E::MissedConditionArgument.link(node));
            }
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
