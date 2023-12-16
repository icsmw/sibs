use crate::reader::{
    chars,
    entry::{Block, Function, Group, Reading, VariableName},
    words, Reader, E,
};
#[derive(Debug)]
pub enum Input {
    VariableName(VariableName),
    Function(Function),
}
#[derive(Debug)]
pub struct Each {
    pub variable: VariableName,
    pub input: Input,
    pub block: Block,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, E> {
        if reader.move_to_word(&[words::EACH])?.is_some() {
            if let Some((inner, uuid)) =
                reader.read_until_close(chars::OPEN_BRACKET, chars::CLOSE_BRACKET, true)?
            {
                if let Some(variable) = VariableName::read(&mut reader.inherit(inner))? {
                    if let Some((inner, _, _)) =
                        reader.read_until(&[chars::OPEN_SQ_BRACKET], false, false)?
                    {
                        let mut inner_reader = reader.inherit(inner);
                        let input =
                            if let Some(variable_name) = VariableName::read(&mut inner_reader)? {
                                Input::VariableName(variable_name)
                            } else if let Some(function) = Function::read(&mut inner_reader)? {
                                Input::Function(function)
                            } else {
                                Err(E::NoLoopInput)?
                            };
                        if let Some(group) = Group::read(reader)? {
                            if reader.move_to_char(&[chars::SEMICOLON])?.is_none() {
                                Err(E::MissedSemicolon)
                            } else {
                                Ok(Some(Each {
                                    variable,
                                    input,
                                    block: Block::read(&mut reader.inherit(group.inner))?
                                        .ok_or(E::EmptyGroup)?,
                                }))
                            }
                        } else {
                            Err(E::NoGroup)
                        }
                    } else {
                        Err(E::NoGroup)
                    }
                } else {
                    Err(E::NoLoopVariable)
                }
            } else {
                Err(E::NoLoopVariable)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test_each {
    use crate::reader::{
        entry::{Each, Reading, E},
        Mapper, Reader,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("../tests/each.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(optional) = Each::read(&mut reader)? {
            println!("{optional:?}");
        }
        println!("_________{}", reader.rest());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
