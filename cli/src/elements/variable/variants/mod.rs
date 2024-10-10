mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{
    error::LinkedErr,
    inf::Value,
    reader::{chars, E},
};

#[derive(Debug, Clone)]
pub struct VariableVariants {
    pub values: Vec<Value>,
    pub token: usize,
}

impl VariableVariants {
    pub fn new(input: String, token: usize) -> Result<Self, LinkedErr<E>> {
        let mut values: Vec<Value> = Vec::new();
        for value in input.split('|') {
            let value = value.trim();
            if !value.is_ascii() {
                Err(E::NotAsciiValue(value.to_string()).linked(&token))?;
            }
            if chars::has_reserved(value) {
                Err(E::UsingReservedChars.linked(&token))?
            }
            if value.is_empty() {
                Err(E::EmptyValue.linked(&token))?;
            }
            if let Ok(num) = value.parse::<isize>() {
                values.push(Value::isize(num))
            } else {
                values.push(Value::String(value.to_string()));
            }
        }
        if values.is_empty() {
            Err(E::NoVariableValues.linked(&token))?;
        }
        Ok(VariableVariants { values, token })
    }
}
