use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};
use std::fmt;

#[derive(Debug)]
pub struct Values {
    pub values: Vec<String>,
    pub token: usize,
}

impl Reading<Values> for Values {
    fn read(reader: &mut Reader) -> Result<Option<Values>, E> {
        let content = reader
            .until()
            .char(&[&chars::SEMICOLON])
            .map(|(content, _)| content)
            .unwrap_or_else(|| reader.move_to().end());
        Ok(Some(Values::new(content, reader.token()?.id)?))
    }
}

impl Values {
    pub fn new(input: String, token: usize) -> Result<Self, E> {
        let mut values: Vec<String> = vec![];
        for value in input.split('|') {
            let value = value.trim();
            if !value.is_ascii() {
                Err(E::NotAsciiValue(value.to_string()))?;
            }
            if value.is_empty() {
                Err(E::EmptyValue)?;
            }
            values.push(value.to_string());
        }
        if values.is_empty() {
            Err(E::NoVariableValues)?;
        }
        Ok(Values { values, token })
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values.join(" | "))
    }
}
