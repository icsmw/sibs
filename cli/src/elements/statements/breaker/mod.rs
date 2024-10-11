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
pub struct Breaker {
    pub token: usize,
}
