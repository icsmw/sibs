use lexer::Kind;

use crate::*;

impl ReadNode<While> for While {
    fn read(parser: &mut Parser, nodes: &Nodes) -> Result<Option<While>, E> {
        let Some(token) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(token.kind, Kind::While) {
            return Ok(None);
        }
        let Some(comparison) =
            Expression::try_oneof(parser, &[ExpressionId::ComparisonSeq], &Nodes::empty())?
                .map(Node::Expression)
        else {
            return Err(E::MissedComparisonInWhile);
        };
        let Some(block) =
            Statement::try_oneof(parser, &[StatementId::Block], nodes)?.map(Node::Statement)
        else {
            return Err(E::MissedBlock);
        };
        Ok(Some(While {
            token,
            comparison: Box::new(comparison),
            block: Box::new(block),
        }))
    }
}
