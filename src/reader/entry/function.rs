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
            if !Reader::is_ascii_alphabetic_and_alphanumeric(
                &name,
                &[&chars::UNDERSCORE, &chars::DASH],
            ) {
                Err(E::InvalidFunctionName)?;
            }
            if matches!(ends_with, Some(chars::SEMICOLON)) {
                reader.move_to().next();
                return Ok(Some(Self::new(reader.token()?.id, None, name, false)?));
            }
            if ends_with.is_none() {
                return Ok(Some(Self::new(
                    reader.token()?.id,
                    Some(reader),
                    name,
                    false,
                )?));
            }
            reader.trim();
            if matches!(ends_with, Some(chars::QUESTION)) {
                reader.move_to().next();
                if let Some(next) = reader.next().char() {
                    if !next.is_whitespace() {
                        Err(E::InvalidFunctionName)?;
                    }
                }
            }
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
                    Some(&mut token.bound),
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
                    Some(&mut token.bound),
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
        mut reader: Option<&mut Reader>,
        name: String,
        tolerance: bool,
    ) -> Result<Self, E> {
        Ok(Self {
            token,
            name,
            tolerance,
            feed: None,
            args: if let Some(reader) = reader.take() {
                Arguments::read(reader)?
            } else {
                None
            },
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
        fn to_string(func: &Function) -> String {
            format!(
                "@{}{}{}",
                func.name,
                if func.tolerance { "?" } else { "" },
                func.args
                    .as_ref()
                    .map(|args| format!(" {args}"))
                    .unwrap_or_default()
            )
        }
        let mut nested: Vec<String> = vec![];
        let mut current = self;
        while let Some(feed) = current.feed.as_ref() {
            nested.push(to_string(feed));
            current = feed;
        }
        nested.reverse();
        write!(
            f,
            "{}{}{}",
            nested.join(" > "),
            if nested.is_empty() { "" } else { " > " },
            to_string(self)
        )
    }
}

#[cfg(test)]
mod test_functions {
    use crate::reader::{
        entry::{Function, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/function.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Function::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 17);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/function.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Function::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
