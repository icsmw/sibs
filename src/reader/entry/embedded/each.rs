use crate::reader::{
    chars,
    entry::{Block, Function, Reading, VariableName},
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
    pub token: usize,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, E> {
        if reader.move_to().word(&[&words::EACH]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                if let Some(variable) = VariableName::read(&mut reader.token()?.walker)? {
                    if reader.until().char(&[&chars::OPEN_SQ_BRACKET]).is_some() {
                        let mut inner_reader = reader.token()?.walker;
                        let input =
                            if let Some(variable_name) = VariableName::read(&mut inner_reader)? {
                                Input::VariableName(variable_name)
                            } else if let Some(function) = Function::read(&mut inner_reader)? {
                                Input::Function(function)
                            } else {
                                Err(E::NoLoopInput)?
                            };
                        if reader
                            .group()
                            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                            .is_some()
                        {
                            let mut token = reader.token()?;
                            if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                                Err(E::MissedSemicolon)
                            } else {
                                Ok(Some(Each {
                                    variable,
                                    input,
                                    block: Block::read(&mut token.walker)?.ok_or(E::EmptyGroup)?,
                                    token: token.id,
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
        Reader,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../tests/each.sibs").to_string());
        let mut count = 0;
        while let Some(optional) = Each::read(&mut reader)? {
            println!("{optional:?}");
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
