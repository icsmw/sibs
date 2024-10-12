mod condition;
mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
mod subsequence;
#[cfg(test)]
mod tests;
mod thread;
mod verification;

pub use condition::*;
pub use subsequence::*;
pub use thread::*;

#[derive(Debug, Clone)]
pub struct If {
    pub threads: Vec<IfThread>,
    pub token: usize,
}
