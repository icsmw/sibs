use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub token: usize,
}

impl Reading<VariableName> for VariableName {
    fn read(reader: &mut Reader) -> Result<Option<VariableName>, E> {
        if reader.move_to().char(&[&chars::DOLLAR]).is_some() {
            let content = reader
                .until()
                .char(&[&chars::COLON, &chars::WS, &chars::EQUAL])
                .map(|(content, _char)| content)
                .unwrap_or_else(|| reader.move_to().end());
            Ok(Some(VariableName::new(content, reader.token()?.id)?))
        } else {
            Ok(None)
        }
    }
}

impl VariableName {
    pub fn new(mut name: String, token: usize) -> Result<Self, E> {
        name = name.trim().to_string();
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
            || name.is_empty()
        {
            Err(E::InvalidVariableName)
        } else {
            Ok(Self { name, token })
        }
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.name)
    }
}

#[cfg(test)]
mod test_variable_name {
    use crate::reader::{
        entry::{Reading, VariableName},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let variables = include_str!("./tests/normal/variable_name.sibs").to_string();
        let variables = variables.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for variable in variables.iter() {
            let mut reader = Reader::new(variable.to_string());
            assert!(VariableName::read(&mut reader).is_ok());
            count += 1;
        }
        assert_eq!(count, variables.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(VariableName::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
