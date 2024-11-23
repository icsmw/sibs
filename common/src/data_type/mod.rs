#[enum_ids::enum_ids]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    Empty,
    // Output(Output),
    // SpawnStatus(SpawnStatus),
    Range(Vec<DataType>),
    Isize(isize),
    F64(f64),
    Bool(bool),
    // PathBuf(PathBuf),
    String(String),
    Vec(Vec<DataType>),
    Error(String),
    // Closure(Uuid),
}
