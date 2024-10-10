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
pub enum Operator {
    Inc,
    Dec,
}

#[derive(Debug, Clone)]
pub struct Incrementer {
    pub variable: Box<Element>,
    pub operator: Operator,
    pub right: Box<Element>,
    pub token: usize,
}
