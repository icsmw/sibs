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
pub enum IfThread {
    // (IfSubsequence, Block)
    If(Box<Element>, Box<Element>),
    // Block
    Else(Box<Element>),
}
