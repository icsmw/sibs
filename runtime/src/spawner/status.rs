#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SpawnStatus {
    Success(Vec<String>),
    Failed(Option<i32>, Vec<String>),
    Cancelled,
}
