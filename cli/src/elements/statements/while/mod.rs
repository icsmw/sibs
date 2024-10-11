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
pub struct While {
    pub condition: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}
