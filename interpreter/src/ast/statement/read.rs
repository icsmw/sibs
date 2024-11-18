use crate::*;

impl AsVec<StatementId> for StatementId {
    fn as_vec() -> Vec<StatementId> {
        StatementId::as_vec()
    }
}

impl Read<Statement, StatementId> for Statement {}

impl TryRead<Statement, StatementId> for Statement {
    fn try_read(parser: &mut Parser, id: StatementId) -> Result<Option<Statement>, LinkedErr<E>> {
        Ok(match id {
            StatementId::Block => Block::read(parser)?.map(Statement::Block),
            StatementId::Break => Break::read(parser)?.map(Statement::Break),
            StatementId::Return => Return::read(parser)?.map(Statement::Return),
            StatementId::Each => Each::read(parser)?.map(Statement::Each),
            StatementId::While => While::read(parser)?.map(Statement::While),
            StatementId::For => For::read(parser)?.map(Statement::For),
            StatementId::Loop => Loop::read(parser)?.map(Statement::Loop),
            StatementId::Assignation => Assignation::read(parser)?.map(Statement::Assignation),
            StatementId::AssignedValue => {
                AssignedValue::read(parser)?.map(Statement::AssignedValue)
            }
            StatementId::Optional => Optional::read(parser)?.map(Statement::Optional),
            StatementId::OneOf => OneOf::read(parser)?.map(Statement::OneOf),
            StatementId::Join => Join::read(parser)?.map(Statement::Join),
            StatementId::If => If::read(parser)?.map(Statement::If),
        })
    }
}
