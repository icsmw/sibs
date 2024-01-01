use crate::reader::{
    chars,
    entry::{Reading, VariableName},
    Reader, E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Input {
    VariableName(VariableName),
    String(String),
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(s) => s.to_string(),
                Self::VariableName(v) => v.to_string(),
            }
        )
    }
}
#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Input>,
    pub token: usize,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to().char(&[&chars::COLON]).is_some() {
            let mut path: Vec<String> = vec![];
            let mut inputs: Vec<Input> = vec![];
            while let Some((content, stopped_on)) =
                reader.until().char(&[&chars::COLON, &chars::SEMICOLON])
            {
                if content.trim().is_empty() {
                    Err(E::EmptyPathToReference)?
                }
                path.push(content);
                reader.move_to().next();
                if stopped_on == chars::SEMICOLON {
                    break;
                }
            }
            if path.pop().is_some() {
                let mut token = reader.token()?;
                let name = token
                    .bound
                    .until()
                    .char(&[&chars::OPEN_BRACKET])
                    .map(|(value, _)| value)
                    .unwrap_or_else(|| token.bound.rest().to_string());
                if token
                    .bound
                    .group()
                    .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                    .is_some()
                {
                    let mut inner = token.bound.token()?.bound;
                    let mut last = false;
                    while let Some(value) = inner
                        .until()
                        .char(&[&chars::COMMA])
                        .map(|(v, _)| {
                            inner.move_to().next();
                            v
                        })
                        .or_else(|| {
                            if !last {
                                last = true;
                                Some(inner.move_to().end())
                            } else {
                                None
                            }
                        })
                    {
                        let mut value_reader = inner.token()?.bound;
                        inputs.push(
                            if let Some(variable_name) = VariableName::read(&mut value_reader)? {
                                Input::VariableName(variable_name)
                            } else {
                                Input::String(value.trim().to_string())
                            },
                        );
                    }
                }
                path.push(name);
            }
            for part in path.iter() {
                if !Reader::is_ascii_alphabetic_and_alphanumeric(
                    part,
                    &[&chars::UNDERSCORE, &chars::DASH],
                ) {
                    Err(E::InvalidReference)?
                }
            }
            Ok(Some(Reference {
                token: reader.token()?.id,
                path,
                inputs,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            ":{}{};",
            self.path.join(":"),
            if self.inputs.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.inputs
                        .iter()
                        .map(|input| input.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        )
    }
}

#[cfg(test)]
mod test_refs {
    use crate::reader::{
        entry::{Reading, Reference},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/refs.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Reference::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/refs.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Reference::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
