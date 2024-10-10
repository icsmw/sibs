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
    reader::{chars, Reader, E},
};

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub token: usize,
}

impl VariableName {
    pub fn new(mut name: String, token: usize) -> Result<Self, LinkedErr<E>> {
        name = name.trim().to_string();
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
            || name.is_empty()
        {
            Err(E::InvalidVariableName.linked(&token))
        } else {
            Ok(Self { name, token })
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
