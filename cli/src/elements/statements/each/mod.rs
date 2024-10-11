mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::elements::Element;

#[derive(Debug, Clone)]
pub struct Each {
    pub variable: Box<Element>,
    pub input: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}
