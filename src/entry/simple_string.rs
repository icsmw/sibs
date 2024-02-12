use std::fmt;

#[derive(Debug, Clone)]
pub struct SimpleString {
    pub value: String,
    pub token: usize,
}

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}
