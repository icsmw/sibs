use crate::parser::{
    chars,
    entry::{Block, Function, Reading, ValueString, VariableAssignation},
    words, Reader, E,
};
use uuid::Uuid;

#[derive(Debug)]
pub enum Action {
    VariableAssignation(VariableAssignation),
    ValueString(ValueString),
    Command(String),
    Block(Block),
}

#[derive(Debug)]
pub struct Optional {
    pub function: Function,
    pub action: Action,
    pub uuid: Uuid,
}

impl Reading<Optional> for Optional {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.hold();
        if reader.move_to_char(chars::AT)? {
            reader.roll_back();
            if let Some((left, _, _)) =
                reader.read_until_word(&[words::DO_ON], &[chars::SEMICOLON], false)?
            {
                if let Some(function) = Function::read(&mut reader.inherit(left))? {
                    if reader.move_to_word(&[words::DO_ON])?.is_some() {
                        if reader.move_to_char(chars::OPEN_SQ_BRACKET)? {
                            if let Some((inner, _, uuid)) =
                                reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
                            {
                                if !reader.move_to_char(chars::SEMICOLON)? {
                                    Err(E::MissedSemicolon)?
                                }
                                if let Some(block) = Block::read(&mut reader.inherit(inner))? {
                                    return Ok(Some(Optional {
                                        uuid,
                                        action: Action::Block(block),
                                        function,
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
                                } else if let Some(value_string) =
                                    ValueString::read(&mut inner_reader)?
                                {
                                    Action::ValueString(value_string)
                                } else {
                                    Action::Command(inner_reader.rest().to_string())
                                },
                                function,
                            }))
                        } else {
                            Err(E::MissedSemicolon)?
                        }
                    } else {
                        Err(E::FailParseOptionalAction)?
                    }
                } else {
                    Err(E::NoFunctionOnOptionalAction)?
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::parser::{
//         entry::{Optional, Reading},
//         Mapper, Reader, E,
//     };

//     #[test]
//     fn reading() -> Result<(), E> {
//         let mut mapper = Mapper::new();
//         let mut reader = Reader::new(
//             include_str!("./tests/optional.sibs").to_string(),
//             &mut mapper,
//             0,
//         );
//         while let Some(optional) = Optional::read(&mut reader)? {
//             println!("{optional:?}");
//         }
//         assert!(reader.rest().trim().is_empty());
//         Ok(())
//     }
// }
