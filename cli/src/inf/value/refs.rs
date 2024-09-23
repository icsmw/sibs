use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Default)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[allow(non_camel_case_types)]
pub enum ValueRef {
    #[default]
    Empty,
    Numeric,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    bool,
    PathBuf,
    String,
    Vec(Box<ValueRef>),
    OneOf(Vec<ValueRef>),
    Optional(Box<ValueRef>),
    Repeated(Box<ValueRef>),
    // (Vec<ARGUMENT_TY>, BLOCK_TY)
    Task(Vec<ValueRef>, Box<ValueRef>),
    Error,
    Closure,
    Output,
    SpawnStatus,
    // Reference to type of input.
    Incoming,
}

impl ValueRef {
    pub fn is_simple(&self) -> bool {
        match self {
            Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize
            | Self::PathBuf
            | Self::String
            | Self::bool
            | Self::Numeric
            | Self::Empty => true,

            Self::OneOf(..)
            | Self::Optional(..)
            | Self::Repeated(..)
            | Self::Task(..)
            | Self::Vec(..)
            | Self::Error
            | Self::Closure
            | Self::Output
            | Self::SpawnStatus
            | Self::Incoming => false,
        }
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize => true,
            Self::OneOf(v) => !v.iter().any(|v| !v.is_numeric()),
            _ => false,
        }
    }
    pub fn is_compatible(&self, other: &ValueRef) -> bool {
        let mut left = self;
        let mut right = other;
        if let (ValueRef::Vec(a), ValueRef::Vec(b)) = (left, right) {
            return a.is_compatible(b);
        } else if !left.is_simple() && !right.is_simple() {
            return left == right;
        } else if (left.is_simple() && !right.is_simple()) || matches!(other, ValueRef::Numeric) {
            right = self;
            left = other;
        }
        match left {
            Self::String
            | Self::SpawnStatus
            | Self::Output
            | Self::bool
            | Self::PathBuf
            | Self::Empty
            | Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize => left == right,
            Self::Numeric => {
                matches!(
                    right,
                    &ValueRef::u8
                        | &ValueRef::u16
                        | &ValueRef::u32
                        | &ValueRef::u64
                        | &ValueRef::u128
                        | &ValueRef::usize
                        | &ValueRef::i8
                        | &ValueRef::i16
                        | &ValueRef::i32
                        | &ValueRef::i64
                        | &ValueRef::i128
                        | &ValueRef::isize
                        | &ValueRef::Numeric
                )
            }
            Self::OneOf(left) => left.contains(right),
            Self::Optional(left) => left.is_compatible(right),
            Self::Vec(left) => {
                if let Self::Vec(right) = right {
                    left.is_compatible(right)
                } else {
                    false
                }
            }
            Self::Repeated(left) => **left == *right,
            Self::Task(..) | Self::Error | Self::Closure | Self::Incoming => false,
        }
    }

    pub fn get_origin(&self) -> Option<&ValueRef> {
        match self {
            Self::Vec(ty) => ty.get_origin(),
            Self::Optional(ty) => ty.get_origin(),
            Self::Repeated(ty) => ty.get_origin(),
            Self::OneOf(tys) => {
                if tys.iter().any(|ty| ty.get_origin().is_none()) {
                    None
                } else {
                    Some(self)
                }
            }
            Self::Incoming => None,
            Self::SpawnStatus
            | Self::Output
            | Self::Task(..)
            | Self::Numeric
            | Self::Error
            | Self::Empty
            | Self::Closure
            | Self::String
            | Self::bool
            | Self::PathBuf
            | Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize => Some(self),
        }
    }
    pub fn into_origin(&self, origin: &ValueRef) -> Option<ValueRef> {
        match self {
            Self::Vec(ty) => ty.into_origin(origin).map(|ty| ValueRef::Vec(Box::new(ty))),
            Self::Optional(ty) => ty
                .into_origin(origin)
                .map(|ty| ValueRef::Optional(Box::new(ty))),
            Self::Repeated(ty) => ty
                .into_origin(origin)
                .map(|ty| ValueRef::Repeated(Box::new(ty))),
            Self::Incoming => origin.get_origin().cloned(),
            Self::SpawnStatus
            | Self::Output
            | Self::OneOf(..)
            | Self::Task(..)
            | Self::Numeric
            | Self::Error
            | Self::Empty
            | Self::Closure
            | Self::String
            | Self::bool
            | Self::PathBuf
            | Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize => Some(self.clone()),
        }
    }
    pub fn has_incoming(&self) -> bool {
        match self {
            Self::Vec(ty) => ty.has_incoming(),
            Self::Optional(ty) => ty.has_incoming(),
            Self::Repeated(ty) => ty.has_incoming(),
            Self::Incoming => true,
            Self::SpawnStatus
            | Self::Output
            | Self::OneOf(..)
            | Self::Task(..)
            | Self::Numeric
            | Self::Error
            | Self::Empty
            | Self::Closure
            | Self::String
            | Self::bool
            | Self::PathBuf
            | Self::u8
            | Self::u16
            | Self::u32
            | Self::u64
            | Self::u128
            | Self::usize
            | Self::i8
            | Self::i16
            | Self::i32
            | Self::i64
            | Self::i128
            | Self::isize => false,
        }
    }
}

pub trait HasOptional {
    fn has_optional(&self) -> bool;
}

impl HasOptional for Vec<ValueRef> {
    fn has_optional(&self) -> bool {
        self.iter().any(|el| matches!(el, ValueRef::Optional(..)))
    }
}
impl HasOptional for &[ValueRef] {
    fn has_optional(&self) -> bool {
        self.iter().any(|el| matches!(el, ValueRef::Optional(..)))
    }
}

pub trait HasRepeated {
    fn has_repeated(&self) -> bool;
}

impl HasRepeated for Vec<ValueRef> {
    fn has_repeated(&self) -> bool {
        self.iter().any(|el| matches!(el, ValueRef::Repeated(..)))
    }
}

impl HasRepeated for &[ValueRef] {
    fn has_repeated(&self) -> bool {
        self.iter().any(|el| matches!(el, ValueRef::Repeated(..)))
    }
}

impl fmt::Display for ValueRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::Empty => "Empty".to_owned(),
                Self::Numeric => "Numeric".to_owned(),
                Self::i8 => "i8".to_owned(),
                Self::i16 => "i16".to_owned(),
                Self::i32 => "i32".to_owned(),
                Self::i64 => "i64".to_owned(),
                Self::i128 => "i128".to_owned(),
                Self::isize => "isize".to_owned(),
                Self::u8 => "u8".to_owned(),
                Self::u16 => "u16".to_owned(),
                Self::u32 => "u32".to_owned(),
                Self::u64 => "u64".to_owned(),
                Self::u128 => "u128".to_owned(),
                Self::usize => "usize".to_owned(),
                Self::bool => "bool".to_owned(),
                Self::PathBuf => "PathBuf".to_owned(),
                Self::String => "String".to_owned(),
                Self::Output => "Output".to_owned(),
                Self::SpawnStatus => "SpawnStatus".to_owned(),
                Self::OneOf(variants) => format!(
                    "OneOf<{}>",
                    variants
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join("."),
                ),
                Self::Optional(ty) => format!("Optional<{ty}>",),
                Self::Repeated(ty) => format!("Repeated<{ty}>",),
                Self::Vec(ty) => format!("Vec<{ty}>",),
                Self::Task(args, block) => format!(
                    "Task<<{}>; <{}>>",
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(";"),
                    block
                ),
                Self::Error => "Error".to_owned(),
                Self::Closure => "Closure".to_owned(),
                Self::Incoming => "Incoming".to_owned(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct PrevValueExpectation {
    pub token: usize,
    pub value: ValueRef,
}
