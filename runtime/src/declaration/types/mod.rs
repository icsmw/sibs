mod entry;
mod parent;
mod scope;
mod store;
mod table;

pub use entry::*;
pub use parent::*;
pub use scope::*;
pub use store::*;
pub use table::*;

use crate::*;
use std::fmt;

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ty {
    /// Can be in If statement. Reflects the fact that the resulting type cannot
    /// be cast to a single type. For example, the branches of an if condition
    /// return different types.
    Indeterminate,
    Determined(DeterminedTy),
    Variants(DeterminedTy),
    OneOf(Vec<DeterminedTy>),
    Optional(DeterminedTy),
    Repeated(DeterminedTy),
    Undefined,
}

impl From<DeterminedTy> for Ty {
    fn from(ty: DeterminedTy) -> Self {
        Ty::Determined(ty)
    }
}

impl Default for Ty {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Ty {
    pub fn reassignable(&self, right: &Ty) -> bool {
        let Self::Determined(right) = right else {
            return false;
        };
        match self {
            Self::Indeterminate => false,
            Self::Determined(ty) => ty.compatible(right),
            Self::Variants(ty) => ty.compatible(right),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            Self::Optional(ty) => ty.compatible(right),
            Self::Repeated(ty) => ty.compatible(right),
            Self::Undefined => true,
        }
    }
    pub fn compatible(&self, right: &Ty) -> bool {
        let Self::Determined(right) = right else {
            return false;
        };
        match self {
            Self::Indeterminate | Self::Undefined => false,
            Self::Determined(ty) => ty.compatible(right),
            Self::Variants(ty) => ty.compatible(right),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            Self::Optional(ty) => ty.compatible(right),
            Self::Repeated(ty) => ty.compatible(right),
        }
    }
    pub fn equal(&self, right: &DeterminedTy) -> bool {
        if let Self::Determined(left) = self {
            left == right
        } else {
            false
        }
    }
    pub fn numeric(&self) -> bool {
        matches!(self, Ty::Determined(DeterminedTy::Num))
    }
    pub fn bool(&self) -> bool {
        matches!(self, Ty::Determined(DeterminedTy::Bool))
    }
    pub fn determined(&self) -> Option<&DeterminedTy> {
        match self {
            Self::Indeterminate
            | Self::Undefined
            | Self::Variants(..)
            | Self::OneOf(..)
            | Self::Optional(..)
            | Self::Repeated(..) => None,
            Self::Determined(ty) => Some(ty),
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Indeterminate => "Indeterminate".to_string(),
                Self::Undefined => "Undefined".to_string(),
                Self::Determined(ty) => format!("Determined:{ty}"),
                Self::Variants(ty) => format!("Variants:{ty}"),
                Self::OneOf(tys) => format!(
                    "OneOf:{}",
                    tys.iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<String>>()
                        .join(" | ")
                ),
                Self::Optional(ty) => format!("Optional:{ty}"),
                Self::Repeated(ty) => format!("Repeated:{ty}"),
            }
        )
    }
}

#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DeterminedTy {
    Recursion(Uuid),
    Void,
    ExecuteResult,
    Range,
    Num,
    Bool,
    PathBuf,
    Str,
    Vec(Option<Box<DeterminedTy>>),
    Error,
    Closure,
}

impl DeterminedTy {
    pub fn compatible(&self, right: &DeterminedTy) -> bool {
        if let (DeterminedTy::Vec(left), DeterminedTy::Vec(right)) = (self, right) {
            if let (Some(left), Some(right)) = (left, right) {
                left == right
            } else {
                left.is_none() && right.is_some()
            }
        } else {
            self == right
        }
    }
}

impl fmt::Display for DeterminedTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
