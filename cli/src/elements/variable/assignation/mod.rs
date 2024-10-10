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
pub struct VariableAssignation {
    pub variable: Box<Element>,
    pub global: bool,
    pub assignation: Box<Element>,
    pub token: usize,
}
