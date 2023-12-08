use uuid::Uuid;

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
    pub uuid: Uuid,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to_char(chars::AT)? {
            if let Some((name, ends_with, uuid)) = reader.read_letters(
                &[chars::CARET, chars::QUESTION, chars::SEMICOLON],
                &[chars::UNDERLINE],
                true,
                false,
            )? {
                if ends_with == chars::SEMICOLON {
                    Ok(Some(Self::new(uuid, reader, name, String::new(), false)?))
                } else if let Some((args, _, uuid)) =
                    reader.read_until(&[chars::SEMICOLON], true, false)?
                {
                    Ok(Some(Self::new(
                        uuid,
                        reader,
                        name,
                        args,
                        ends_with == chars::QUESTION,
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

// #[cfg(test)]
// mod test {
//     use crate::parser::{
//         entry::{Function, Reading},
//         Mapper, Reader, E,
//     };

//     #[test]
//     fn reading() -> Result<(), E> {
//         let mut mapper = Mapper::new();
//         let mut reader = Reader::new(
//             include_str!("./tests/function.sibs").to_string(),
//             &mut mapper,
//             0,
//         );
//         while let Some(func) = Function::read(&mut reader)? {
//             println!("{func:?}");
//         }
//         println!("{:?}", reader.mapper);
//         assert!(reader.rest().trim().is_empty());
//         Ok(())
//     }
// }
