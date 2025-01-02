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

    pub fn as_ty(&self) -> Option<Ty> {
        match self {
            Self::Num(..) => Some(DeterminatedTy::Num.into()),
            Self::Bool(..) => Some(DeterminatedTy::Bool.into()),
            Self::PathBuf(..) => Some(DeterminatedTy::PathBuf.into()),
            Self::Str(..) => Some(DeterminatedTy::Str.into()),
            Self::Range(..) => Some(DeterminatedTy::Range.into()),
            Self::Error => Some(DeterminatedTy::Error.into()),
            Self::ExecuteResult => Some(DeterminatedTy::ExecuteResult.into()),
            Self::Closure => Some(DeterminatedTy::Closure.into()),
            Self::Void => Some(DeterminatedTy::Void.into()),
            Self::Vec(els) => {
                if let Some(el) = els.first() {
                    if let Some(Ty::Determinated(ty)) = el.as_ty() {
                        Some(DeterminatedTy::Vec(Some(Box::new(ty))).into())
                    } else {
                        None
                    }
                } else {
                    Some(DeterminatedTy::Vec(None).into())
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
