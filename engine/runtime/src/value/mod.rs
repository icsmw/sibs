mod converting;

pub use converting::*;

use crate::{spawner::SpawnStatus, *};
use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum ExecuteResult {
    Success(Vec<RtValue>),
    Failed(Option<i32>, Vec<RtValue>),
    RunError(String),
    Cancelled,
}

impl ExecuteResult {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(..))
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(..) | Self::RunError(..))
    }
    pub fn is_cancelled(&self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

impl fmt::Display for ExecuteResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Success(..) => "Success".to_owned(),
                Self::Failed(code, ..) =>
                    if let Some(code) = code {
                        format!("Failed (code {code})")
                    } else {
                        "Failed".to_owned()
                    },
                Self::RunError(err) => format!("Fail to run: {err}"),
                Self::Cancelled => "Cancelled".to_owned(),
            }
        )
    }
}

impl From<SpawnStatus> for ExecuteResult {
    fn from(status: SpawnStatus) -> Self {
        match status {
            SpawnStatus::Success(output) => {
                ExecuteResult::Success(output.into_iter().map(RtValue::Str).collect())
            }
            SpawnStatus::Failed(code, output) => {
                ExecuteResult::Failed(code, output.into_iter().map(RtValue::Str).collect())
            }
            SpawnStatus::RunError(err) => ExecuteResult::RunError(err),
            SpawnStatus::Cancelled => ExecuteResult::Cancelled,
        }
    }
}

/// Runtime Value
#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq)]
pub enum RtValue {
    Void,
    ExecuteResult(ExecuteResult),
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
    NamedArgumentValue(String, Box<RtValue>),
    Skipped,
}

impl From<SpawnStatus> for RtValue {
    fn from(status: SpawnStatus) -> Self {
        RtValue::ExecuteResult(status.into())
    }
}

impl RtValue {
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::ExecuteResult(..)
            | Self::NamedArgumentValue(..)
            | Self::Error
            | Self::Closure(..)
            | Self::BinaryOperator(..)
            | Self::ComparisonOperator(..)
            | Self::LogicalOperator(..)
            | Self::Skipped => None,
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
            Self::ExecuteResult(..) => Some(DeterminedTy::ExecuteResult.into()),
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
            Self::BinaryOperator(..)
            | Self::LogicalOperator(..)
            | Self::ComparisonOperator(..)
            | Self::NamedArgumentValue(..)
            | Self::Skipped => None,
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
            | Self::ExecuteResult(..)
            | Self::Range(..)
            | Self::Vec(..)
            | Self::NamedArgumentValue(..)
            | Self::Void
            | Self::Skipped => None,
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
                Self::ExecuteResult(v) => format!("ExecuteResult({v})"),
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
                Self::NamedArgumentValue(n, v) => format!("NamedArgumentValue({n}: {v})"),
                Self::Skipped => String::from("Skipped"),
            }
        )
    }
}
