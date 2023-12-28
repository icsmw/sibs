use crate::reader::{
    chars,
    entry::{Arguments, Reading},
    words, Reader, E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Function {
    pub tolerance: bool,
    pub name: String,
    pub args: Option<Arguments>,
    pub feed: Option<Box<Function>>,
    pub token: usize,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.state().set();
        if reader.move_to().char(&[&chars::AT]).is_some() {
            let (name, ends_with) = reader
                .until()
                .char(&[
                    &chars::CARET,
                    &chars::QUESTION,
                    &chars::SEMICOLON,
                    &chars::WS,
                ])
                .map(|(str, char)| (str, Some(char)))
                .unwrap_or_else(|| (reader.move_to().end(), None));
            if matches!(ends_with, Some(chars::SEMICOLON)) {
                reader.move_to().next();
                return Ok(Some(Self::new(
                    reader.token()?.id,
                    &mut reader.token()?.bound,
                    name,
                    false,
                )?));
            }
            if ends_with.is_none() {
                return Ok(Some(Self::new(reader.token()?.id, reader, name, false)?));
            }
            reader.trim();
            let stop_on = reader
                .until()
                .char(&[&chars::REDIRECT, &chars::SEMICOLON])
                .map(|(_, stop_on)| Some(stop_on))
                .unwrap_or_else(|| {
                    reader.move_to().end();
                    None
                });
            let mut token = reader.token()?;
            if token.bound.contains().word(&[&words::DO_ON]) {
                let _ = reader.state().restore();
                return Ok(None);
            }
            reader.move_to().next();
            if matches!(stop_on, Some(chars::REDIRECT)) {
                let feed = Self::new(
                    token.id,
                    &mut token.bound,
                    name,
                    matches!(ends_with, Some(chars::QUESTION)),
                )?;
                if let Some(mut parent_func) = Function::read(reader)? {
                    parent_func.feeding(feed);
                    Ok(Some(parent_func))
                } else {
                    Err(E::NoDestFunction)
                }
            } else {
                Ok(Some(Self::new(
                    token.id,
                    &mut token.bound,
                    name,
                    matches!(ends_with, Some(chars::QUESTION)),
                )?))
            }
        } else {
            Ok(None)
        }
    }
}

impl Function {
    pub fn new(
        token: usize,
        reader: &mut Reader,
        name: String,
        tolerance: bool,
    ) -> Result<Self, E> {
        Ok(Self {
            token,
            name,
            tolerance,
            feed: None,
            args: Arguments::read(reader)?,
        })
    }
    pub fn feeding(&mut self, func: Function) {
        if let Some(bound) = self.feed.as_mut() {
            bound.feeding(func);
        } else {
            self.feed = Some(Box::new(func));
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "@{}{}{}",
            self.name,
            if self.tolerance { "?" } else { "" },
            self.args
                .as_ref()
                .map(|args| format!(" {args}"))
                .unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod test_functions {
    use crate::reader::{
        entry::{Function, Reading},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/function.sibs").to_string());
        let mut count = 0;
        while let Some(func) = Function::read(&mut reader)? {
            println!("{func:?}");
            count += 1;
        }
        assert_eq!(count, 17);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
