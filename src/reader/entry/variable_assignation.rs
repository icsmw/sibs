use crate::reader::{
    chars,
    entry::{Block, First, Function, Group, Reader, Reading, ValueString, VariableName},
    E,
};

#[derive(Debug)]
pub enum Assignation {
    Function(Function),
    ValueString(ValueString),
    Block(Block),
    First(First),
}
#[derive(Debug)]
pub struct VariableAssignation {
    pub name: VariableName,
    pub assignation: Assignation,
    pub index: usize,
}

impl Reading<VariableAssignation> for VariableAssignation {
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, E> {
        reader.hold();
        let from = reader.pos;
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to_char(&[chars::EQUAL])?.is_some() {
                if let Some(chars::EQUAL) = reader.next_char() {
                    // This is condition
                    reader.roll_back();
                    return Ok(None);
                }
                if let Some(first) = First::read(reader)? {
                    if reader.move_to_char(&[chars::SEMICOLON])?.is_none() {
                        Err(E::MissedSemicolon)
                    } else {
                        Ok(Some(VariableAssignation {
                            name,
                            assignation: Assignation::First(first),
                            index: reader.get_index_until_current(from),
                        }))
                    }
                } else if let Some(group) = Group::read(reader)? {
                    if reader.move_to_char(&[chars::SEMICOLON])?.is_none() {
                        Err(E::MissedSemicolon)
                    } else {
                        Ok(Some(VariableAssignation {
                            name,
                            assignation: Assignation::Block(
                                Block::read(&mut reader.inherit(group.inner))?
                                    .ok_or(E::EmptyGroup)?,
                            ),
                            index: reader.get_index_until_current(from),
                        }))
                    }
                } else if let Some((inner, _, _)) =
                    reader.read_until(&[chars::SEMICOLON], true, true)?
                {
                    let mut inner_reader = reader.inherit(inner);
                    if let Some(func) = Function::read(&mut inner_reader)? {
                        Ok(Some(VariableAssignation {
                            name,
                            assignation: Assignation::Function(func),
                            index: reader.get_index_until_current(from),
                        }))
                    } else if let Some(value_string) = ValueString::read(&mut inner_reader)? {
                        Ok(Some(VariableAssignation {
                            name,
                            assignation: Assignation::ValueString(value_string),
                            index: reader.get_index_until_current(from),
                        }))
                    } else {
                        Err(E::NoComparingOrAssignation)?
                    }
                } else {
                    Err(E::MissedSemicolon)
                }
            } else {
                Err(E::NoComparingOrAssignation)?
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test_variable_assignation {
    use crate::reader::{
        entry::{Reading, VariableAssignation},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/variable_assignation.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(task) = VariableAssignation::read(&mut reader)? {
            println!("{task:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
