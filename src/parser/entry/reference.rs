use uuid::Uuid;

use crate::parser::{
    chars,
    entry::{Argument, Arguments, Reading},
    words, Reader, E,
};

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub uuid: Uuid,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::COLON)? {
            let mut path: Vec<String> = vec![];
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
            Ok(Some(Reference {
                uuid: Uuid::new_v4(),
                path,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
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
