use crate::*;

/// Runtime Value
#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum RtValue {
    Void,
    ExecuteResult,
    Range(RangeInclusive<isize>),
    Num(f64),
    Bool(bool),
    PathBuf(PathBuf),
    Str(String),
    Vec(Vec<RtValue>),
    Error,
    Closure,
    BinaryOperator(BinaryOperator),
    ComparisonOperator(ComparisonOperator),
    LogicalOperator(LogicalOperator),
}

impl RtValue {
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::ExecuteResult
            | Self::Error
            | Self::Closure
            | Self::BinaryOperator(..)
            | Self::ComparisonOperator(..)
            | Self::LogicalOperator(..) => None,
            Self::Bool(v) => Some(v.to_string()),
            Self::Num(v) => Some(v.to_string()),
            Self::Str(v) => Some(v),
            Self::Void => Some(String::new()),
            Self::PathBuf(v) => Some(v.to_string_lossy().to_string()),
            Self::Range(v) => Some(format!("{}..{}", v.start(), v.end())),
            Self::Vec(v) => {
                let vls = v
                    .into_iter()
                    .map(|v| v.as_string())
                    .collect::<Vec<Option<String>>>();
                if vls.iter().any(|v| v.is_none()) {
                    None
                } else {
                    Some(
                        vls.into_iter()
                            .map(|v| v.unwrap_or_default())
                            .collect::<Vec<String>>()
                            .join(", "),
                    )
                }
            }
        }
    }

    pub fn as_ty(&self) -> Option<DataType> {
        match self {
            Self::Num(..) => Some(DataType::Num),
            Self::Bool(..) => Some(DataType::Bool),
            Self::PathBuf(..) => Some(DataType::PathBuf),
            Self::Str(..) => Some(DataType::Str),
            Self::Range(..) => Some(DataType::Range),
            Self::Error => Some(DataType::Error),
            Self::ExecuteResult => Some(DataType::ExecuteResult),
            Self::Closure => Some(DataType::Closure),
            Self::Void => Some(DataType::Void),
            Self::Vec(els) => {
                if let Some(el) = els.first() {
                    el.as_ty().map(|ty| DataType::Vec(Box::new(ty)))
                } else {
                    Some(DataType::Vec(Box::new(DataType::Undefined)))
                }
            }
            Self::BinaryOperator(..) | Self::LogicalOperator(..) | Self::ComparisonOperator(..) => {
                None
            }
        }
    }

    pub fn into_eq_ord(self) -> Option<RtValueEqOrd> {
        match self {
            Self::Num(v) => Some(RtValueEqOrd::Num(v)),
            Self::Bool(v) => Some(RtValueEqOrd::Bool(v)),
            Self::PathBuf(v) => Some(RtValueEqOrd::PathBuf(v)),
            Self::Str(v) => Some(RtValueEqOrd::Str(v)),
            Self::BinaryOperator(..)
            | Self::LogicalOperator(..)
            | Self::Closure
            | Self::ComparisonOperator(..)
            | Self::Error
            | Self::ExecuteResult
            | Self::Range(..)
            | Self::Vec(..)
            | Self::Void => None,
        }
    }
}

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RtValueEqOrd {
    Num(f64),
    Bool(bool),
    PathBuf(PathBuf),
    Str(String),
}
