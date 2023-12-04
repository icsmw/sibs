use crate::parser::{
    chars,
    entry::{Arguments, Reading},
    Reader, E,
};

#[derive(Debug)]
pub struct Function {
    pub tolerance: bool,
    pub name: String,
    pub args: Option<Arguments>,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::AT)? {
            if let Some((word, ends_with)) =
                reader.read_letters(&[chars::CARET, chars::QUESTION, chars::SEMICOLON], true)?
            {
                if ends_with == chars::SEMICOLON {
                    Ok(Some(Self::new(word, String::new(), false)?))
                } else if let Some((args, _)) = reader.read_until(&[chars::SEMICOLON], true)? {
                    Ok(Some(Self::new(word, args, ends_with == chars::QUESTION)?))
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
    pub fn new(name: String, args: String, tolerance: bool) -> Result<Self, E> {
        let mut reader = Reader::new(args);
        Ok(Self {
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
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/function.sibs").to_string());
        while let Some(func) = Function::read(&mut reader)? {
            println!("{func:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
