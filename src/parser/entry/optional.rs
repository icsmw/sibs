use crate::parser::{
    chars,
    entry::{Block, Function, Reading, ValueString, VariableAssignation, VariableComparing},
    words, Reader, E,
};
use uuid::Uuid;

use super::variable_comparing;

#[derive(Debug)]
pub enum Action {
    VariableAssignation(VariableAssignation),
    ValueString(ValueString),
    Command(String),
    Block(Block),
}

#[derive(Debug)]
pub enum Condition {
    Function(Function),
    VariableComparing(VariableComparing),
}

#[derive(Debug)]
pub struct Optional {
    pub condition: Condition,
    pub action: Action,
    pub uuid: Uuid,
}

impl Reading<Optional> for Optional {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.hold();
        if reader.move_to_char(&[chars::AT, chars::DOLLAR])?.is_some() {
            reader.roll_back();
            if let Some((left, _, _)) =
                reader.read_until_word(&[words::DO_ON], &[chars::SEMICOLON], false)?
            {
                let condition = if let Some(variable_comparing) =
                    VariableComparing::read(&mut reader.inherit(left.clone()))?
                {
                    Condition::VariableComparing(variable_comparing)
                } else if let Some(function) = Function::read(&mut reader.inherit(left))? {
                    Condition::Function(function)
                } else {
                    Err(E::NoFunctionOnOptionalAction)?
                };
                if reader.move_to_word(&[words::DO_ON])?.is_some() {
                    if reader.move_to_char(&[chars::OPEN_SQ_BRACKET])?.is_some() {
                        if let Some((inner, _, uuid)) =
                            reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
                        {
                            if !reader.move_to_char(&[chars::SEMICOLON])?.is_some() {
                                Err(E::MissedSemicolon)?
                            }
                            if let Some(block) = Block::read(&mut reader.inherit(inner))? {
                                return Ok(Some(Optional {
                                    uuid,
                                    action: Action::Block(block),
                                    condition,
                                }));
                            }
                        } else {
                            Err(E::NotClosedGroup)?
                        }
                    }
                    if let Some((inner, _, uuid)) =
                        reader.read_until(&[chars::SEMICOLON], true, false)?
                    {
                        let mut inner_reader = reader.inherit(inner);
                        Ok(Some(Optional {
                            uuid,
                            action: if let Some(assignation) =
                                VariableAssignation::read(&mut inner_reader)?
                            {
                                Action::VariableAssignation(assignation)
                            } else if let Some(value_string) = ValueString::read(&mut inner_reader)?
                            {
                                Action::ValueString(value_string)
                            } else {
                                Action::Command(inner_reader.rest().to_string())
                            },
                            condition,
                        }))
                    } else {
                        Err(E::MissedSemicolon)?
                    }
                } else {
                    Err(E::FailParseOptionalAction)?
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        entry::{Optional, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/optional.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(optional) = Optional::read(&mut reader)? {
            println!("{optional:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
