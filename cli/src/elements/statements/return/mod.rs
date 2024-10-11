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
pub struct Return {
    pub token: usize,
    pub output: Option<Box<Element>>,
}
