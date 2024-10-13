use crate::elements::{Element, Metadata};

mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

#[derive(Debug, Clone)]
pub struct SimpleString {
    pub value: String,
    pub token: usize,
}

impl SimpleString {
    pub fn as_element<S: AsRef<str>>(content: S, token: &usize) -> Element {
        let mut md = Metadata::default();
        md.set_token(*token);
        Element::SimpleString(
            SimpleString {
                value: content.as_ref().to_string(),
                token: *token,
            },
            md,
        )
    }
}
