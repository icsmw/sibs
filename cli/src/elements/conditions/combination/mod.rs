mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cmb {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Combination {
    pub cmb: Cmb,
    pub token: usize,
}
