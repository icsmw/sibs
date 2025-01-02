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
    Indeterminated,
    Determinated(DeterminatedTy),
    Variants(DeterminatedTy),
    OneOf(Vec<DeterminatedTy>),
    Optional(DeterminatedTy),
    Repeated(DeterminatedTy),
    Undefined,
}

impl From<DeterminatedTy> for Ty {
    fn from(ty: DeterminatedTy) -> Self {
        Ty::Determinated(ty)
    }
}

impl Default for Ty {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Ty {
    pub fn reassignable(&self, right: &Ty) -> bool {
        let Self::Determinated(right) = right else {
            return false;
        };
        match self {
            Self::Indeterminated => false,
            Self::Determinated(ty) => ty.compatible(right),
            Self::Variants(ty) => ty.compatible(right),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            Self::Optional(ty) => ty.compatible(right),
            Self::Repeated(ty) => ty.compatible(right),
            Self::Undefined => true,
        }
    }
    pub fn compatible(&self, right: &Ty) -> bool {
        let Self::Determinated(right) = right else {
            return false;
        };
        match self {
            Self::Indeterminated | Self::Undefined => false,
            Self::Determinated(ty) => ty.compatible(right),
            Self::Variants(ty) => ty.compatible(right),
            Self::OneOf(tys) => tys.iter().any(|ty| ty.compatible(right)),
            Self::Optional(ty) => ty.compatible(right),
            Self::Repeated(ty) => ty.compatible(right),
        }
    }
    pub fn equal(&self, right: &DeterminatedTy) -> bool {
        if let Self::Determinated(left) = self {
            left == right
        } else {
            false
        }
    }
    pub fn numeric(&self) -> bool {
        matches!(self, Ty::Determinated(DeterminatedTy::Num))
    }
    pub fn bool(&self) -> bool {
        matches!(self, Ty::Determinated(DeterminatedTy::Bool))
    }
    pub fn determinated(&self) -> Option<&DeterminatedTy> {
        match self {
            Self::Indeterminated
            | Self::Undefined
            | Self::Variants(..)
            | Self::OneOf(..)
            | Self::Optional(..)
            | Self::Repeated(..) => None,
            Self::Determinated(ty) => Some(ty),
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Indeterminated => "Indeterminated".to_string(),
                Self::Undefined => "Undefined".to_string(),
                Self::Determinated(ty) => format!("Determinated:{ty}"),
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
pub enum DeterminatedTy {
    Recursion(Uuid),
    Void,
    ExecuteResult,
    Range,
    Num,
    Bool,
    PathBuf,
    Str,
    Vec(Option<Box<DeterminatedTy>>),
    Error,
    Closure,
}

impl DeterminatedTy {
    pub fn compatible(&self, right: &DeterminatedTy) -> bool {
        if let (DeterminatedTy::Vec(left), DeterminatedTy::Vec(right)) = (self, right) {
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

impl fmt::Display for DeterminatedTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id())
    }
}
