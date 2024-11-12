use lexer::Kind;

use crate::*;

impl ReadNode<Assignation> for Assignation {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<Assignation>, E> {
        let Some(left) =
            Expression::try_oneof(parser, &[ExpressionId::Variable], nodes)?.map(Node::Expression)
        else {
            return Ok(None);
        };
        let Some(right) =
            Statement::try_read(parser, StatementId::AssignedValue, nodes)?.map(Node::Statement)
        else {
            return Ok(None);
        };
        Ok(Some(Assignation {
            left: Box::new(left),
            right: Box::new(right),
        }))
    }
}
