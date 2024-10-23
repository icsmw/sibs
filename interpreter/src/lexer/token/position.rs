#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub from: usize,
    pub to: usize,
}

impl Position {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}
