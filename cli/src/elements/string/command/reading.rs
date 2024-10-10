use crate::{
    elements::{string, Command, ElementRef},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Command> for Command {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Command>, LinkedErr<E>> {
        if let Some((_, elements, token)) = string::read(reader, chars::TILDA, ElementRef::Command)?
        {
            Ok(Some(Command { elements, token }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Command, Command> for Command {}
