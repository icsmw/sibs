mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Comment {
    pub comment: String,
    pub token: usize,
}
