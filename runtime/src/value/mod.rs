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
}

impl RtValue {
    pub fn as_string(self) -> Option<String> {
        match self {
            Self::ExecuteResult
            | Self::Error
            | Self::Closure
            | Self::BinaryOperator(..)
            | Self::ComparisonOperator(..) => None,
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
}
