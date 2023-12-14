use crate::parser::{
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
    Combination(Combination),
    Group(Vec<Proviso>),
}

#[derive(Debug)]
pub enum Element {
    If(Vec<Proviso>, Block),
    Else(Block),
}
#[derive(Debug)]
pub struct Condition {
    pub elements: Vec<Element>,
}

impl Reading<Condition> for Condition {
    fn read(reader: &mut Reader) -> Result<Option<Condition>, E> {
        let mut elements: Vec<Element> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to_word(&[words::IF])?.is_some() {
                if let Some((inner, _char, _uuid)) =
                    reader.read_until(&[chars::OPEN_SQ_BRACKET], true, false)?
                {
                    let mut inner_reader = reader.inherit(inner);
                    let proviso: Vec<Proviso> = Condition::proviso(&mut inner_reader)?;
                    if let Some((block, _, _)) =
                        reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
                    {
                        if let Some(block) = Block::read(&mut reader.inherit(block))? {
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
            if reader.move_to_char(&[chars::SEMICOLON])?.is_some() {
                return Ok(Some(Condition { elements }));
            }
            if reader.move_to_word(&[words::ELSE])?.is_some() {
                if reader.move_to_char(&[chars::OPEN_SQ_BRACKET])?.is_some() {
                    if let Some((block, _, _)) =
                        reader.read_until(&[chars::CLOSE_SQ_BRACKET], true, false)?
                    {
                        if let Some(block) = Block::read(&mut reader.inherit(block))? {
                            elements.push(Element::Else(block));
                        } else {
                            Err(E::EmptyGroup)?
                        }
                        if reader.move_to_char(&[chars::SEMICOLON])?.is_some() {
                            return Ok(Some(Condition { elements }));
                        } else {
                            Err(E::MissedSemicolon)?
                        }
                    } else {
                        Err(E::NotClosedGroup)?
                    }
                } else {
                    Err(E::NoGroup)?
                }
            }
        }
        Ok(None)
    }
}

impl Condition {
    pub fn inner(reader: &mut Reader) -> Result<Proviso, E> {
        if let Some(variable_name) = VariableName::read(reader)? {
            if let Some(word) = reader.move_to_word(&[words::CMP_TRUE, words::CMP_FALSE])? {
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
        let negative = reader.move_to_char(&[chars::EXCLAMATION])?.is_some();
        if let Some(func) = Function::read(reader)? {
            Ok(Proviso::Function(func, negative))
        } else {
            Err(E::NoProvisoOfCondition)
        }
    }
    pub fn proviso(reader: &mut Reader) -> Result<Vec<Proviso>, E> {
        let mut proviso: Vec<Proviso> = vec![];
        while !reader.rest().trim().is_empty() {
            if reader.move_to_char(&[chars::OPEN_BRACKET])?.is_some() {
                if let Some((group, _, _)) =
                    reader.read_until(&[chars::CLOSE_BRACKET], true, false)?
                {
                    let mut group_reader = reader.inherit(group);
                    if group_reader.move_to_char(&[chars::OPEN_BRACKET])?.is_some() {
                        Err(E::NestedConditionGroups)?
                    }
                    proviso.push(Proviso::Group(Condition::proviso(&mut group_reader)?));
                    continue;
                } else {
                    Err(E::NotClosedConditionGroup)?
                }
            }
            if let Some(combination) = reader.move_to_word(&[words::AND, words::OR])? {
                if let Some(Proviso::Combination(_)) = proviso.last() {
                    Err(E::RepeatedCombinationOperator)?
                } else if proviso.last().is_none() {
                    Err(E::NoProvisoOfCondition)?
                }
                proviso.push(Proviso::Combination(if combination == words::AND {
                    Combination::And
                } else {
                    Combination::Or
                }));
            }
            if let Some((inner, combination, uuid)) =
                reader.read_until_word(&[words::AND, words::OR], &[], true)?
            {
                proviso.push(Condition::inner(&mut reader.inherit(inner))?);
                proviso.push(Proviso::Combination(if combination == words::AND {
                    Combination::And
                } else {
                    Combination::Or
                }));
            } else {
                proviso.push(Condition::inner(reader)?);
            }
        }
        Ok(proviso)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        entry::{Condition, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/conditions.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(task) = Condition::read(&mut reader)? {
            println!("{task:?}");
        }

        println!("{}", reader.rest().trim());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
