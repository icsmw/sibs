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
            StatementId::Block => Block::read(parser, nodes)?.map(Statement::Block),
            StatementId::Break => Break::read(parser, nodes)?.map(Statement::Break),
            StatementId::Return => Return::read(parser, nodes)?.map(Statement::Return),
            StatementId::Each => Each::read(parser, nodes)?.map(Statement::Each),
            StatementId::While => While::read(parser, nodes)?.map(Statement::While),
            StatementId::For => For::read(parser, nodes)?.map(Statement::For),
            StatementId::Loop => Loop::read(parser, nodes)?.map(Statement::Loop),
            StatementId::Assignation => {
                Assignation::read(parser, nodes)?.map(Statement::Assignation)
            }
            StatementId::AssignedValue => {
                AssignedValue::read(parser, nodes)?.map(Statement::AssignedValue)
            }
            StatementId::Optional => Optional::read(parser, nodes)?.map(Statement::Optional),
            StatementId::OneOf => OneOf::read(parser, nodes)?.map(Statement::OneOf),
            StatementId::Join => Join::read(parser, nodes)?.map(Statement::Join),
            StatementId::If => If::read(parser, nodes)?.map(Statement::If),
        })
    }
}
