mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Meta {
    pub inner: Vec<String>,
    pub token: usize,
}

impl Meta {
    pub fn as_string(&self) -> String {
        self.inner.join("\n")
    }
    pub fn as_lines(&self) -> Vec<&str> {
        self.inner.iter().map(|s| s.as_str()).collect()
    }
}
