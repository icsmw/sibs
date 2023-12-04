use crate::parser::E;
use std::fmt;

#[derive(Debug)]
pub struct Values {
    pub values: Vec<String>,
}

impl Values {
    pub fn new(input: String) -> Result<Self, E> {
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
        Ok(Values { values })
    }
}
