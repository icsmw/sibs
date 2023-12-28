use crate::reader::{
    chars,
    entry::{Block, Function, Reading, ValueString, VariableName},
    words, Reader, E,
};

#[derive(Debug, Clone)]
pub enum Cmp {
    Equal,
    NotEqual,
}

#[derive(Debug, Clone)]
pub enum Combination {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Proviso {
    Variable(VariableName, Cmp, ValueString),
    // Function, is negative (!)
    Function(Function, bool),
    Combination(Combination, usize),
    Group(Vec<Proviso>),
}

#[derive(Debug)]
pub enum Element {
    If(Vec<Proviso>, Block),
    Else(Block),
}
#[derive(Debug)]
pub struct If {
    pub elements: Vec<Element>,
    pub index: usize,
}

impl Reading<If> for If {
    fn read(reader: &mut Reader) -> Result<Option<If>, E> {
        let mut elements: Vec<Element> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[&words::IF]).is_some() {
                if reader.until().char(&[&chars::OPEN_SQ_BRACKET]).is_some() {
                    let proviso: Vec<Proviso> = If::proviso(&mut reader.token()?.walker)?;
                    if reader
                        .group()
                        .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                        .is_some()
                    {
                        if let Some(block) = Block::read(&mut reader.token()?.walker)? {
                            elements.push(Element::If(proviso, block));
                        } else {
                            Err(E::EmptyGroup)?
                        }
                    } else {
                        Err(E::NotClosedGroup)?
                    }
                    continue;
                } else {
                    Err(E::NoGroup)?
                }
            }
            if elements.is_empty() {
                return Ok(None);
            }
            if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                return Ok(Some(If {
                    elements,
                    index: reader.token()?.id,
                }));
            }
            if reader.move_to().word(&[&words::ELSE]).is_some() {
                if reader
                    .group()
                    .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                    .is_some()
                {
                    if let Some(block) = Block::read(&mut reader.token()?.walker)? {
                        elements.push(Element::Else(block));
                    } else {
                        Err(E::EmptyGroup)?
                    }
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_some() {
                        return Ok(Some(If {
                            elements,
                            index: reader.token()?.id,
                        }));
                    } else {
                        Err(E::MissedSemicolon)?
                    }
                } else {
                    Err(E::NoGroup)?
                }
            }
        }
        Ok(None)
    }
}

impl If {
    pub fn inner(reader: &mut Reader) -> Result<Proviso, E> {
        if let Some(variable_name) = VariableName::read(reader)? {
            if let Some(word) = reader
                .move_to()
                .word(&[&words::CMP_TRUE, &words::CMP_FALSE])
            {
                if let Some(value_string) = ValueString::read(reader)? {
                    return Ok(Proviso::Variable(
                        variable_name,
                        if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value_string,
                    ));
                } else {
                    Err(E::NoStringValueWithCondition)?
                }
            } else {
                Err(E::MissedComparingOperator)?
            }
        }
        let negative = reader.move_to().char(&[&chars::EXCLAMATION]).is_some();
        if let Some(func) = Function::read(reader)? {
            Ok(Proviso::Function(func, negative))
        } else {
            Err(E::NoProvisoOfCondition)
        }
    }
    pub fn proviso(reader: &mut Reader) -> Result<Vec<Proviso>, E> {
        let mut proviso: Vec<Proviso> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to().char(&[&chars::OPEN_BRACKET]).is_some() {
                if reader.until().char(&[&chars::CLOSE_BRACKET]).is_some() {
                    let mut group_reader = reader.token()?.walker;
                    if group_reader
                        .move_to()
                        .char(&[&chars::OPEN_BRACKET])
                        .is_some()
                    {
                        Err(E::NestedConditionGroups)?
                    }
                    proviso.push(Proviso::Group(If::proviso(&mut group_reader)?));
                    continue;
                } else {
                    Err(E::NotClosedConditionGroup)?
                }
            }
            if let Some(combination) = reader.move_to().word(&[&words::AND, words::OR]) {
                if let Some(Proviso::Combination(_, _)) = proviso.last() {
                    Err(E::RepeatedCombinationOperator)?
                } else if proviso.last().is_none() {
                    Err(E::NoProvisoOfCondition)?
                }
                proviso.push(Proviso::Combination(
                    if combination == words::AND {
                        Combination::And
                    } else {
                        Combination::Or
                    },
                    reader.token()?.id,
                ));
            }
            if let Some((_, combination)) = reader.until().word(&[&words::AND, &words::OR]) {
                let mut token = reader.token()?;
                if !reader.move_to().whitespace() {
                    Err(E::NoWhitespaceAfterCondition)?;
                }
                proviso.push(If::inner(&mut token.walker)?);
                proviso.push(Proviso::Combination(
                    if combination == words::AND {
                        Combination::And
                    } else {
                        Combination::Or
                    },
                    token.id,
                ));
            } else {
                proviso.push(If::inner(reader)?);
            }
        }
        Ok(proviso)
    }
}

#[cfg(test)]
mod test_if {
    use crate::reader::{
        entry::{If, Reading},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../tests/if.sibs").to_string());
        let mut count = 0;
        while let Some(task) = If::read(&mut reader)? {
            println!("{task:?}");
            count += 1;
        }
        assert_eq!(count, 10);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
