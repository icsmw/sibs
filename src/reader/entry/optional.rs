use crate::reader::{
    chars,
    entry::{Block, Function, Reading, ValueString, VariableAssignation, VariableComparing},
    words, Reader, E,
};

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
    pub token: usize,
}

impl Reading<Optional> for Optional {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.state().set();
        if reader
            .move_to()
            .char(&[&chars::AT, &chars::DOLLAR])
            .is_some()
        {
            reader.state().restore()?;
            if reader
                .cancel_on(&[&chars::SEMICOLON, &chars::OPEN_SQ_BRACKET])
                .until()
                .word(&[words::DO_ON])
                .is_some()
            {
                let mut token = reader.token()?;
                let condition =
                    if let Some(variable_comparing) = VariableComparing::read(&mut token.walker)? {
                        Condition::VariableComparing(variable_comparing)
                    } else if let Some(function) = Function::read(&mut token.walker)? {
                        Condition::Function(function)
                    } else {
                        Err(E::NoFunctionOnOptionalAction)?
                    };
                if reader.move_to().word(&[&words::DO_ON]).is_some() {
                    if reader
                        .group()
                        .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                        .is_some()
                    {
                        let mut token = reader.token()?;
                        if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                            Err(E::MissedSemicolon)?
                        }
                        if let Some(block) = Block::read(&mut token.walker)? {
                            return Ok(Some(Optional {
                                token: token.id,
                                action: Action::Block(block),
                                condition,
                            }));
                        } else {
                            Err(E::InvalidBlock)?
                        }
                    }
                    if reader.until().char(&[&chars::SEMICOLON]).is_some() {
                        let mut token = reader.token()?;
                        reader.move_to().next();
                        Ok(Some(Optional {
                            token: token.id,
                            action: if let Some(assignation) =
                                VariableAssignation::read(&mut token.walker)?
                            {
                                Action::VariableAssignation(assignation)
                            } else if let Some(value_string) = ValueString::read(&mut token.walker)?
                            {
                                Action::ValueString(value_string)
                            } else {
                                Action::Command(token.walker.rest().to_string())
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
mod test_optional {
    use crate::reader::{
        entry::{Optional, Reading},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/optional.sibs").to_string());
        let mut count = 0;
        while let Some(optional) = Optional::read(&mut reader)? {
            println!("{optional:?}");
            count += 1;
        }
        assert_eq!(count, 9);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
