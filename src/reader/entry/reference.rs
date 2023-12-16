use uuid::Uuid;

use crate::{
    functions::reader,
    reader::{
        chars,
        entry::{Reading, VariableName},
        words, Reader, E,
    },
};

#[derive(Debug, Clone)]
pub enum Input {
    VariableName(VariableName),
    String(String),
}
#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Input>,
    pub uuid: Uuid,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(&[chars::COLON])?.is_some() {
            let mut path: Vec<String> = vec![];
            let mut inputs: Vec<Input> = vec![];
            while let Some((content, stopped_on, _)) =
                reader.read_until(&[chars::COLON, chars::SEMICOLON], true, false)?
            {
                if content.trim().is_empty() {
                    Err(E::EmptyPathToReference)?
                }
                path.push(content);
                if stopped_on == chars::SEMICOLON {
                    break;
                }
            }
            if let Some(last) = path.pop() {
                let mut inner_reader = reader.inherit(last);
                if let Some((name, stopped_on, _)) =
                    inner_reader.read_until(&[chars::OPEN_BRACKET], false, true)?
                {
                    if stopped_on == chars::OPEN_BRACKET {
                        if let Some((content, _)) = inner_reader.read_until_close(
                            chars::OPEN_BRACKET,
                            chars::CLOSE_BRACKET,
                            true,
                        )? {
                            for value in content.split(',') {
                                inputs.push(
                                    if let Some(variable_name) = VariableName::read(
                                        &mut inner_reader.inherit(value.trim().to_string()),
                                    )? {
                                        Input::VariableName(variable_name)
                                    } else {
                                        Input::String(value.trim().to_string())
                                    },
                                );
                            }
                        }
                    } else {
                        path.push(name);
                    }
                }
            }
            Ok(Some(Reference {
                uuid: Uuid::new_v4(),
                path,
                inputs,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test_refs {
    use crate::reader::{
        entry::{Reading, Reference},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/refs.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(refs) = Reference::read(&mut reader)? {
            println!("{refs:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
