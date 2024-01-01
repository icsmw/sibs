use crate::reader::{
    chars,
    entry::{Block, First, Function, Reader, Reading, ValueString, VariableName},
    E,
};
use std::fmt;

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
    pub token: usize,
}

impl Reading<VariableAssignation> for VariableAssignation {
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, E> {
        reader.state().set();
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to().char(&[&chars::EQUAL]).is_some() {
                if let Some(chars::EQUAL) = reader.next().char() {
                    // This is condition
                    reader.state().restore()?;
                    return Ok(None);
                }
                let assignation = if let Some(first) = First::read(reader)? {
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon)?;
                    } else {
                        Some(VariableAssignation {
                            name: name.clone(),
                            assignation: Assignation::First(first),
                            token: reader.token()?.id,
                        })
                    }
                } else if reader
                    .group()
                    .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                    .is_some()
                {
                    let mut group_token = reader.token()?;
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon)?;
                    } else {
                        Some(VariableAssignation {
                            name: name.clone(),
                            assignation: Assignation::Block(
                                Block::read(&mut group_token.bound)?.ok_or(E::EmptyGroup)?,
                            ),
                            token: group_token.id,
                        })
                    }
                } else {
                    None
                };
                if assignation.is_some() {
                    reader.move_to().next();
                    return Ok(assignation);
                }
                let mut token = reader
                    .until()
                    .char(&[&chars::SEMICOLON])
                    .map(|_| {
                        reader.move_to().next();
                        reader.token()
                    })
                    .unwrap_or_else(|| {
                        let _ = reader.move_to().end();
                        reader.token()
                    })?;
                if let Some(func) = Function::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::Function(func),
                        token: token.id,
                    }))
                } else if let Some(value_string) = ValueString::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::ValueString(value_string),
                        token: token.id,
                    }))
                } else {
                    Err(E::NoComparingOrAssignation)?
                }
            } else {
                Err(E::NoComparingOrAssignation)?
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for VariableAssignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = {};",
            self.name,
            match &self.assignation {
                Assignation::ValueString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            }
        )
    }
}

#[cfg(test)]
mod test_variable_assignation {
    use crate::reader::{
        entry::{Reading, VariableAssignation},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("./tests/normal/variable_assignation.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
