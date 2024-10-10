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
    Div,
    Mlt,
}

#[derive(Debug, Clone)]
pub struct Compute {
    pub left: Box<Element>,
    pub operator: Operator,
    pub right: Box<Element>,
    pub token: usize,
}
