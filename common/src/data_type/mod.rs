use std::{fmt, path::Display};

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    /// Cabe be in If statement
    IndeterminateType,
    Void,
    // Output(Output),
    SpawnStatus,
    Range,
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

impl DataType {
    pub fn compatible(&self, other: &DataType) -> bool {
        if self == other {
            return true;
        }
        match self {
            Self::F64 | Self::Isize => matches!(other, DataType::F64 | DataType::Isize),
            _ => false,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
