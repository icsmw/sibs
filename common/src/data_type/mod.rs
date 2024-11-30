use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    /// Can be in If statement
    IndeterminateType,
    Void,
    // Output(Output),
    SpawnStatus,
    Range,
    Isize,
    F64,
    Bool,
    PathBuf,
    Str,
    Vec(Box<DataType>),
    Error,
    Closure,
    Variants(Box<DataType>),
    Undefined,
    /// bool | str
    OneOf(Vec<DataType>),
}

impl DataType {
    pub fn reassignable(&self, right: &DataType) -> bool {
        if matches!(right, Self::Undefined) {
            return false;
        }
        if self == right {
            return true;
        }
        if matches!(self, Self::Undefined) {
            return true;
        }
        if let (DataType::Vec(left), DataType::Vec(right)) = (self, right) {
            return left.reassignable(right);
        }
        match self {
            Self::F64 | Self::Isize => matches!(right, DataType::F64 | DataType::Isize),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            _ => false,
        }
    }
    pub fn compatible(&self, right: &DataType) -> bool {
        if self == right {
            return true;
        }
        match self {
            Self::F64 | Self::Isize => matches!(right, DataType::F64 | DataType::Isize),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            _ => false,
        }
    }
    pub fn numeric(&self) -> bool {
        matches!(self, DataType::F64 | DataType::Isize)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
