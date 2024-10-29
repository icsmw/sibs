use crate::*;

impl AsVec<StatementId> for StatementId {
    fn as_vec() -> Vec<StatementId> {
        StatementId::as_vec()
    }
}

impl Read<Statement, StatementId> for Statement {}

impl TryRead<Statement, StatementId> for Statement {
    fn try_read(
        parser: &mut Parser,
        id: StatementId,
        nodes: &Nodes,
    ) -> Result<Option<Statement>, E> {
        Ok(match id {
            StatementId::Break => Break::read(parser, nodes)?.map(Statement::Break),
            StatementId::Return => Return::read(parser, nodes)?.map(Statement::Return),
        })
    }
}
