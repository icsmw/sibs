mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: isize,
    pub token: usize,
}
