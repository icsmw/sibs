use uuid::Uuid;

use crate::parser::{
    chars,
    entry::{Arguments, Reading},
    words, Reader, E,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub tolerance: bool,
    pub name: String,
    pub args: Option<Arguments>,
    pub uuid: Uuid,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.hold();
        if reader.move_to_char(chars::AT)? {
            if let Some((name, ends_with, uuid)) = reader.read_letters(
                &[chars::CARET, chars::QUESTION, chars::SEMICOLON],
                &[chars::UNDERLINE],
                true,
            )? {
                if let Some(chars::SEMICOLON) = ends_with {
                    return Ok(Some(Self::new(uuid, reader, name, String::new(), false)?));
                }
                if ends_with.is_none() {
                    return Ok(Some(Self::new(uuid, reader, name, String::new(), false)?));
                }
                if let Some((args, _, uuid)) = reader.read_until(&[chars::SEMICOLON], true, true)? {
                    if reader.inherit(args.clone()).has_word(&[words::DO_ON])? {
                        reader.roll_back();
                        return Ok(None);
                    }
                    Ok(Some(Self::new(
                        uuid,
                        reader,
                        name,
                        args,
                        matches!(ends_with, Some(chars::QUESTION)),
                    )?))
                } else {
                    Err(E::MissedSemicolon)
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl Function {
    pub fn new(
        uuid: Uuid,
        parent: &mut Reader,
        name: String,
        args: String,
        tolerance: bool,
    ) -> Result<Self, E> {
        let mut reader = parent.inherit(args);
        Ok(Self {
            uuid,
            name,
            tolerance,
            args: Arguments::read(&mut reader)?,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        entry::{Function, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/function.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(func) = Function::read(&mut reader)? {
            println!("{func:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
