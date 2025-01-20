mod conflict;

mod assignation;
mod assigned_value;
mod block;
mod r#break;
mod r#for;
mod r#if;
mod join;
mod r#loop;
mod oneof;
mod optional;
mod r#return;
mod r#while;

use crate::*;

impl AsVec<StatementId> for StatementId {
    fn as_vec() -> Vec<StatementId> {
        StatementId::as_vec()
    }
}

impl TryRead<Statement, StatementId> for Statement {
    fn try_read(parser: &mut Parser, id: StatementId) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(match id {
            StatementId::Block => Block::read_as_linked(parser)?,
            StatementId::Break => Break::read_as_linked(parser)?,
            StatementId::Return => Return::read_as_linked(parser)?,
            StatementId::While => While::read_as_linked(parser)?,
            StatementId::For => For::read_as_linked(parser)?,
            StatementId::Loop => Loop::read_as_linked(parser)?,
            StatementId::Assignation => Assignation::read_as_linked(parser)?,
            StatementId::AssignedValue => AssignedValue::read_as_linked(parser)?,
            StatementId::Optional => Optional::read_as_linked(parser)?,
            StatementId::OneOf => OneOf::read_as_linked(parser)?,
            StatementId::Join => Join::read_as_linked(parser)?,
            StatementId::If => If::read_as_linked(parser)?,
        })
    }
}
