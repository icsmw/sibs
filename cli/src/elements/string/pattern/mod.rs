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

#[derive(Debug, Clone, Default)]
pub struct PatternString {
    pub elements: Vec<Element>,
    pub token: usize,
}
