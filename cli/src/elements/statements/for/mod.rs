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
pub struct For {
    pub index: Box<Element>,
    pub target: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}
