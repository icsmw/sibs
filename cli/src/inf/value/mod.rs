mod convertor;
#[cfg(test)]
mod test;
use crate::{elements::Cmb, inf::journal::Output};
use std::{any::TypeId, fmt, path::PathBuf};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[allow(non_camel_case_types)]
pub enum ValueRef {
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
}

#[cfg(test)]
impl Default for ValueRef {
    fn default() -> Self {
        ValueRef::Empty
    }
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
            | Self::Vec(..) => false,
        }
    }
    pub fn is_compatible(&self, other: &ValueRef) -> bool {
        let mut left = self;
        let mut right = other;
        if (left.is_simple() && !right.is_simple()) || matches!(other, ValueRef::Numeric) {
            right = self;
            left = other;
        }
        match left {
            Self::String
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
            Self::Task(..) => false,
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
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum Value {
    Empty(()),
    Output(Output),
    Cmb(Cmb),
    i8(i8),
    i16(i16),
    i32(i32),
    i64(i64),
    i128(i128),
    isize(isize),
    u8(u8),
    u16(u16),
    u32(u32),
    u64(u64),
    u128(u128),
    usize(usize),
    bool(bool),
    PathBuf(PathBuf),
    String(String),
    Vec(Vec<Value>),
}

impl Value {
    pub fn not_empty_or<E>(self, err: E) -> Result<Value, E> {
        if self.is_empty() {
            Err(err)
        } else {
            Ok(self)
        }
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty(..))
    }

    pub fn empty() -> Self {
        Self::Empty(())
    }

    pub fn duplicate(&self) -> Self {
        self.clone()
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self {
            Value::Output(v) if TypeId::of::<T>() == TypeId::of::<Output>() => {
                Some(unsafe { &*(v as *const Output as *const T) })
            }
            Value::Cmb(v) if TypeId::of::<T>() == TypeId::of::<Cmb>() => {
                Some(unsafe { &*(v as *const Cmb as *const T) })
            }
            Value::i8(v) if TypeId::of::<T>() == TypeId::of::<i8>() => {
                Some(unsafe { &*(v as *const i8 as *const T) })
            }
            Value::i16(v) if TypeId::of::<T>() == TypeId::of::<i16>() => {
                Some(unsafe { &*(v as *const i16 as *const T) })
            }
            Value::i32(v) if TypeId::of::<T>() == TypeId::of::<i32>() => {
                Some(unsafe { &*(v as *const i32 as *const T) })
            }
            Value::i64(v) if TypeId::of::<T>() == TypeId::of::<i64>() => {
                Some(unsafe { &*(v as *const i64 as *const T) })
            }
            Value::i128(v) if TypeId::of::<T>() == TypeId::of::<i128>() => {
                Some(unsafe { &*(v as *const i128 as *const T) })
            }
            Value::isize(v) if TypeId::of::<T>() == TypeId::of::<isize>() => {
                Some(unsafe { &*(v as *const isize as *const T) })
            }
            Value::u8(v) if TypeId::of::<T>() == TypeId::of::<u8>() => {
                Some(unsafe { &*(v as *const u8 as *const T) })
            }
            Value::u16(v) if TypeId::of::<T>() == TypeId::of::<u16>() => {
                Some(unsafe { &*(v as *const u16 as *const T) })
            }
            Value::u32(v) if TypeId::of::<T>() == TypeId::of::<u32>() => {
                Some(unsafe { &*(v as *const u32 as *const T) })
            }
            Value::u64(v) if TypeId::of::<T>() == TypeId::of::<u64>() => {
                Some(unsafe { &*(v as *const u64 as *const T) })
            }
            Value::u128(v) if TypeId::of::<T>() == TypeId::of::<u128>() => {
                Some(unsafe { &*(v as *const u128 as *const T) })
            }
            Value::usize(v) if TypeId::of::<T>() == TypeId::of::<usize>() => {
                Some(unsafe { &*(v as *const usize as *const T) })
            }
            Value::bool(v) if TypeId::of::<T>() == TypeId::of::<bool>() => {
                Some(unsafe { &*(v as *const bool as *const T) })
            }
            Value::PathBuf(v) if TypeId::of::<T>() == TypeId::of::<PathBuf>() => {
                Some(unsafe { &*(v as *const PathBuf as *const T) })
            }
            Value::String(v) if TypeId::of::<T>() == TypeId::of::<String>() => {
                Some(unsafe { &*(v as *const String as *const T) })
            }
            Value::Vec(v) if TypeId::of::<T>() == TypeId::of::<Vec<Value>>() => {
                Some(unsafe { &*(v as *const Vec<Value> as *const T) })
            }
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self {
            Value::Cmb(v) if TypeId::of::<T>() == TypeId::of::<Cmb>() => {
                Some(unsafe { &mut *(v as *mut Cmb as *mut T) })
            }
            Value::i8(v) if TypeId::of::<T>() == TypeId::of::<i8>() => {
                Some(unsafe { &mut *(v as *mut i8 as *mut T) })
            }
            Value::i16(v) if TypeId::of::<T>() == TypeId::of::<i16>() => {
                Some(unsafe { &mut *(v as *mut i16 as *mut T) })
            }
            Value::i32(v) if TypeId::of::<T>() == TypeId::of::<i32>() => {
                Some(unsafe { &mut *(v as *mut i32 as *mut T) })
            }
            Value::i64(v) if TypeId::of::<T>() == TypeId::of::<i64>() => {
                Some(unsafe { &mut *(v as *mut i64 as *mut T) })
            }
            Value::i128(v) if TypeId::of::<T>() == TypeId::of::<i128>() => {
                Some(unsafe { &mut *(v as *mut i128 as *mut T) })
            }
            Value::isize(v) if TypeId::of::<T>() == TypeId::of::<isize>() => {
                Some(unsafe { &mut *(v as *mut isize as *mut T) })
            }
            Value::u8(v) if TypeId::of::<T>() == TypeId::of::<u8>() => {
                Some(unsafe { &mut *(v as *mut u8 as *mut T) })
            }
            Value::u16(v) if TypeId::of::<T>() == TypeId::of::<u16>() => {
                Some(unsafe { &mut *(v as *mut u16 as *mut T) })
            }
            Value::u32(v) if TypeId::of::<T>() == TypeId::of::<u32>() => {
                Some(unsafe { &mut *(v as *mut u32 as *mut T) })
            }
            Value::u64(v) if TypeId::of::<T>() == TypeId::of::<u64>() => {
                Some(unsafe { &mut *(v as *mut u64 as *mut T) })
            }
            Value::u128(v) if TypeId::of::<T>() == TypeId::of::<u128>() => {
                Some(unsafe { &mut *(v as *mut u128 as *mut T) })
            }
            Value::usize(v) if TypeId::of::<T>() == TypeId::of::<usize>() => {
                Some(unsafe { &mut *(v as *mut usize as *mut T) })
            }
            Value::bool(v) if TypeId::of::<T>() == TypeId::of::<bool>() => {
                Some(unsafe { &mut *(v as *mut bool as *mut T) })
            }
            Value::PathBuf(v) if TypeId::of::<T>() == TypeId::of::<PathBuf>() => {
                Some(unsafe { &mut *(v as *mut PathBuf as *mut T) })
            }
            Value::String(v) if TypeId::of::<T>() == TypeId::of::<String>() => {
                Some(unsafe { &mut *(v as *mut String as *mut T) })
            }
            Value::Vec(v) if TypeId::of::<T>() == TypeId::of::<Vec<Value>>() => {
                Some(unsafe { &mut *(v as *mut Vec<Value> as *mut T) })
            }
            _ => None,
        }
    }

    pub fn as_num(&self) -> Option<isize> {
        match self {
            Self::i8(v) => Some(*v as isize),
            Self::i16(v) => Some(*v as isize),
            Self::i32(v) => Some(*v as isize),
            Self::i64(v) => Some(*v as isize),
            Self::i128(v) => Some(*v as isize),
            Self::isize(v) => Some(*v),
            Self::u8(v) => Some(*v as isize),
            Self::u16(v) => Some(*v as isize),
            Self::u32(v) => Some(*v as isize),
            Self::u64(v) => Some(*v as isize),
            Self::u128(v) => Some(*v as isize),
            Self::usize(v) => Some(*v as isize),
            Self::String(v) => v.parse::<isize>().ok(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::i8(v) => Some(v.to_string()),
            Self::i16(v) => Some(v.to_string()),
            Self::i32(v) => Some(v.to_string()),
            Self::i64(v) => Some(v.to_string()),
            Self::i128(v) => Some(v.to_string()),
            Self::isize(v) => Some(v.to_string()),
            Self::u8(v) => Some(v.to_string()),
            Self::u16(v) => Some(v.to_string()),
            Self::u32(v) => Some(v.to_string()),
            Self::u64(v) => Some(v.to_string()),
            Self::u128(v) => Some(v.to_string()),
            Self::usize(v) => Some(v.to_string()),
            Self::bool(v) => Some(v.to_string()),
            Self::PathBuf(v) => Some(v.display().to_string()),
            Self::String(v) => Some(v.clone()),
            _ => None,
        }
    }

    pub fn as_path_buf(&self) -> Option<PathBuf> {
        match self {
            Self::PathBuf(v) => Some(v.clone()),
            Self::String(v) => Some(PathBuf::from(v)),
            _ => None,
        }
    }

    pub fn as_strings(&self) -> Option<Vec<String>> {
        match self {
            Self::Vec(inner) => {
                let mut strings: Vec<String> = Vec::new();
                for value in inner.iter() {
                    if let Some(s) = value.as_string() {
                        strings.push(s);
                    } else {
                        return None;
                    }
                }
                Some(strings)
            }
            _ => None,
        }
    }

    pub fn as_path_bufs(&self) -> Option<Vec<PathBuf>> {
        match self {
            Self::Vec(inner) => {
                let mut paths: Vec<PathBuf> = Vec::new();
                for value in inner.iter() {
                    if let Some(s) = value.as_path_buf() {
                        paths.push(s);
                    } else {
                        return None;
                    }
                }
                Some(paths)
            }
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::i8(v) => Some(v == &1),
            Self::i16(v) => Some(v == &1),
            Self::i32(v) => Some(v == &1),
            Self::i64(v) => Some(v == &1),
            Self::i128(v) => Some(v == &1),
            Self::isize(v) => Some(v == &1),
            Self::u8(v) => Some(v == &1),
            Self::u16(v) => Some(v == &1),
            Self::u32(v) => Some(v == &1),
            Self::u64(v) => Some(v == &1),
            Self::u128(v) => Some(v == &1),
            Self::usize(v) => Some(v == &1),
            Self::bool(v) => Some(*v),
            Self::String(v) => Some(v.to_lowercase() == "true"),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
