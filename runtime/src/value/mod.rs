mod converting;

pub use converting::*;

use crate::*;
use std::fmt;

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
    Closure(Uuid),
    BinaryOperator(BinaryOperator),
    ComparisonOperator(ComparisonOperator),
    LogicalOperator(LogicalOperator),
}

impl RtValue {
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::ExecuteResult
            | Self::Error
            | Self::Closure(..)
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

    pub fn as_ty(&self) -> Option<Ty> {
        match self {
            Self::Num(..) => Some(DeterminedTy::Num.into()),
            Self::Bool(..) => Some(DeterminedTy::Bool.into()),
            Self::PathBuf(..) => Some(DeterminedTy::PathBuf.into()),
            Self::Str(..) => Some(DeterminedTy::Str.into()),
            Self::Range(..) => Some(DeterminedTy::Range.into()),
            Self::Error => Some(DeterminedTy::Error.into()),
            Self::ExecuteResult => Some(DeterminedTy::ExecuteResult.into()),
            Self::Closure(uuid) => Some(DeterminedTy::Closure(*uuid, None).into()),
            Self::Void => Some(DeterminedTy::Void.into()),
            Self::Vec(els) => {
                if let Some(el) = els.first() {
                    if let Some(Ty::Determined(ty)) = el.as_ty() {
                        Some(DeterminedTy::Vec(Some(Box::new(ty))).into())
                    } else {
                        None
                    }
                } else {
                    Some(DeterminedTy::Vec(None).into())
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
            | Self::Closure(..)
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

impl fmt::Display for RtValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => String::from("Void"),
                Self::ExecuteResult => String::from("ExecuteResult"),
                Self::Range(v) => format!("Range({v:?})"),
                Self::Num(v) => format!("Num({v})"),
                Self::Bool(v) => format!("Bool({v})"),
                Self::PathBuf(v) => format!("PathBuf({})", v.to_string_lossy()),
                Self::Str(v) => format!("Str({v})"),
                Self::Vec(v) => format!("Vec({v:?})"),
                Self::Error => String::from("Error"),
                Self::Closure(v) => format!("Closure({v})"),
                Self::BinaryOperator(..) => String::from("BinaryOperator"),
                Self::ComparisonOperator(..) => String::from("ComparisonOperator"),
                Self::LogicalOperator(..) => String::from("LogicalOperator"),
            }
        )
    }
}
