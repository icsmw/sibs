mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{
    elements::Element,
    reader::{words, E},
};

#[derive(Debug, Clone)]
pub enum Cmp {
    Equal,
    NotEqual,
    LeftBig,
    RightBig,
    LeftBigInc,
    RightBigInc,
}

impl Cmp {
    pub fn from_str(value: &str) -> Result<Self, E> {
        match value {
            words::CMP_TRUE => Ok(Self::Equal),
            words::CMP_FALSE => Ok(Self::NotEqual),
            words::CMP_RBIG => Ok(Self::RightBig),
            words::CMP_LBIG => Ok(Self::LeftBig),
            words::CMP_LBIG_INC => Ok(Self::LeftBigInc),
            words::CMP_RBIG_INC => Ok(Self::RightBigInc),
            _ => Err(E::UnrecognizedCode(value.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Comparing {
    pub left: Box<Element>,
    pub cmp: Cmp,
    pub right: Box<Element>,
    pub token: usize,
}
