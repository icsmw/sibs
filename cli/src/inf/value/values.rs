use crate::{
    elements::Cmb,
    inf::{journal::Output, operator, ValueRef},
};
use std::{any::TypeId, fmt, path::PathBuf};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum Value {
    Empty(()),
    Output(Output),
    Range(Vec<Value>),
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
    pub fn as_ref(&self) -> Result<ValueRef, operator::E> {
        Ok(match self {
            Self::Empty(..) => ValueRef::Empty,
            Self::Output(..) => ValueRef::Empty, // <== TODO: Replace with some kind of ValueRef::Inner
            Self::Cmb(..) => ValueRef::Empty, // <== TODO: Replace with some kind of ValueRef::Inner
            Self::Range(..) => ValueRef::Vec(Box::new(ValueRef::isize)),
            Self::i8(..) => ValueRef::i8,
            Self::i16(..) => ValueRef::i16,
            Self::i32(..) => ValueRef::i32,
            Self::i64(..) => ValueRef::i64,
            Self::i128(..) => ValueRef::i128,
            Self::isize(..) => ValueRef::isize,
            Self::u8(..) => ValueRef::u8,
            Self::u16(..) => ValueRef::u16,
            Self::u32(..) => ValueRef::u32,
            Self::u64(..) => ValueRef::u64,
            Self::u128(..) => ValueRef::u128,
            Self::usize(..) => ValueRef::usize,
            Self::bool(..) => ValueRef::bool,
            Self::PathBuf(..) => ValueRef::PathBuf,
            Self::String(..) => ValueRef::String,
            Self::Vec(v) => ValueRef::Vec(Box::new(
                v.first()
                    .and_then(|v| v.as_ref().ok())
                    .ok_or(operator::E::EmptyVector)?,
            )),
        })
    }
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
            Value::Range(v) if TypeId::of::<T>() == TypeId::of::<Vec<Value>>() => {
                Some(unsafe { &*(v as *const Vec<Value> as *const T) })
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

#[derive(Debug, Clone)]
pub struct PrevValue {
    pub token: usize,
    pub value: Value,
}
