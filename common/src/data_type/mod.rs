#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    Void,
    // Output(Output),
    SpawnStatus,
    Range(Vec<DataType>),
    Isize,
    F64,
    Bool,
    PathBuf,
    String,
    Vec(Box<DataType>),
    Error,
    Closure,
    Variants(Box<DataType>),
    Undefined,
}
