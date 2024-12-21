use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    /// Can be in If statement. Reflects the fact that the resulting type cannot
    /// be cast to a single type. For example, the branches of an if condition
    /// return different types.
    IndeterminateType,
    Void,
    ExecuteResult,
    Range,
    Num,
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
    Optional(Box<DataType>),
    Repeated(Box<DataType>),
}

impl Default for DataType {
    fn default() -> Self {
        Self::Void
    }
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
            Self::Num => matches!(right, DataType::Num),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            _ => false,
        }
    }
    pub fn compatible(&self, right: &DataType) -> bool {
        if self == right {
            return true;
        }
        match self {
            Self::Num => matches!(right, DataType::Num),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            _ => false,
        }
    }
    pub fn numeric(&self) -> bool {
        matches!(self, DataType::Num)
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
