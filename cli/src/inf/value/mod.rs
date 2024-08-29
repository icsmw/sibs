mod convertor;
mod error;
use crate::{elements::Cmb, inf::journal::Output};
pub use error::E;
use std::{any::TypeId, fmt, path::PathBuf};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[allow(non_camel_case_types)]
pub enum ValueRef {
    // Special type to redirect to inside element. For example block, which returns result of last element inside
    Inner,
    Any,
    Empty,
    Output,
    Cmb,
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

impl fmt::Display for ValueRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}",
            match self {
                Self::Inner => "Inner".to_owned(),
                Self::Any => "Any".to_owned(),
                Self::Empty => "Empty".to_owned(),
                Self::Output => "Output".to_owned(),
                Self::Cmb => "Cmb".to_owned(),
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn as_num() {
        assert_eq!(Value::i8(1).as_num(), Some(1));
        assert_eq!(Value::i16(1).as_num(), Some(1));
        assert_eq!(Value::i32(1).as_num(), Some(1));
        assert_eq!(Value::i64(1).as_num(), Some(1));
        assert_eq!(Value::i128(1).as_num(), Some(1));
        assert_eq!(Value::isize(1).as_num(), Some(1));
        assert_eq!(Value::u8(1).as_num(), Some(1));
        assert_eq!(Value::u16(1).as_num(), Some(1));
        assert_eq!(Value::u32(1).as_num(), Some(1));
        assert_eq!(Value::u64(1).as_num(), Some(1));
        assert_eq!(Value::u128(1).as_num(), Some(1));
        assert_eq!(Value::usize(1).as_num(), Some(1));
        assert_eq!(Value::String(String::from("123")).as_num(), Some(123));
        assert_eq!(Value::String(String::from("abc")).as_num(), None);
        assert_eq!(Value::bool(true).as_num(), None);
        assert_eq!(Value::PathBuf(PathBuf::from("test")).as_num(), None);
        assert_eq!(Value::Vec(vec![Value::i8(1)]).as_num(), None);
    }

    #[test]
    fn as_string() {
        assert_eq!(Value::i8(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::i16(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::i32(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::i64(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::i128(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::isize(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::u8(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::u16(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::u32(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::u64(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::u128(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::usize(1).as_string(), Some("1".to_string()));
        assert_eq!(Value::bool(true).as_string(), Some("true".to_string()));
        assert_eq!(
            Value::PathBuf(PathBuf::from("test")).as_string(),
            Some("test".to_string())
        );
        assert_eq!(
            Value::String(String::from("test")).as_string(),
            Some("test".to_string())
        );
        assert_eq!(Value::Vec(vec![Value::i8(1)]).as_string(), None);
    }

    #[test]
    fn as_bool() {
        // Testing integer types
        assert_eq!(Value::i8(1).as_bool(), Some(true));
        assert_eq!(Value::i8(0).as_bool(), Some(false));
        assert_eq!(Value::i16(1).as_bool(), Some(true));
        assert_eq!(Value::i16(0).as_bool(), Some(false));
        assert_eq!(Value::i32(1).as_bool(), Some(true));
        assert_eq!(Value::i32(0).as_bool(), Some(false));
        assert_eq!(Value::i64(1).as_bool(), Some(true));
        assert_eq!(Value::i64(0).as_bool(), Some(false));
        assert_eq!(Value::i128(1).as_bool(), Some(true));
        assert_eq!(Value::i128(0).as_bool(), Some(false));
        assert_eq!(Value::isize(1).as_bool(), Some(true));
        assert_eq!(Value::isize(0).as_bool(), Some(false));
        assert_eq!(Value::u8(1).as_bool(), Some(true));
        assert_eq!(Value::u8(0).as_bool(), Some(false));
        assert_eq!(Value::u16(1).as_bool(), Some(true));
        assert_eq!(Value::u16(0).as_bool(), Some(false));
        assert_eq!(Value::u32(1).as_bool(), Some(true));
        assert_eq!(Value::u32(0).as_bool(), Some(false));
        assert_eq!(Value::u64(1).as_bool(), Some(true));
        assert_eq!(Value::u64(0).as_bool(), Some(false));
        assert_eq!(Value::u128(1).as_bool(), Some(true));
        assert_eq!(Value::u128(0).as_bool(), Some(false));
        assert_eq!(Value::usize(1).as_bool(), Some(true));
        assert_eq!(Value::usize(0).as_bool(), Some(false));

        // Testing bool type
        assert_eq!(Value::bool(true).as_bool(), Some(true));
        assert_eq!(Value::bool(false).as_bool(), Some(false));

        // Testing String type
        assert_eq!(Value::String(String::from("true")).as_bool(), Some(true));
        assert_eq!(Value::String(String::from("false")).as_bool(), Some(false));
        assert_eq!(Value::String(String::from("TRUE")).as_bool(), Some(true));
        assert_eq!(Value::String(String::from("FALSE")).as_bool(), Some(false));
        assert_eq!(Value::String(String::from("TrUe")).as_bool(), Some(true));
        assert_eq!(Value::String(String::from("FaLsE")).as_bool(), Some(false));
        assert_eq!(Value::String(String::from("yes")).as_bool(), Some(false));
        assert_eq!(Value::String(String::from("no")).as_bool(), Some(false));

        // Testing other types should return None
        assert_eq!(Value::PathBuf(PathBuf::from("test")).as_bool(), None);
        assert_eq!(Value::Vec(vec![Value::i8(1)]).as_bool(), None);
    }

    #[test]
    fn as_path_buf() {
        assert_eq!(
            Value::PathBuf(PathBuf::from("test")).as_path_buf(),
            Some(PathBuf::from("test"))
        );
        assert_eq!(
            Value::String(String::from("test")).as_path_buf(),
            Some(PathBuf::from("test"))
        );
        assert_eq!(Value::i8(1).as_path_buf(), None);
    }

    #[test]
    fn as_strings() {
        assert_eq!(
            Value::Vec(vec![
                Value::String(String::from("test1")),
                Value::String(String::from("test2"))
            ])
            .as_strings(),
            Some(vec![String::from("test1"), String::from("test2")])
        );
        assert_eq!(
            Value::Vec(vec![Value::String(String::from("test1")), Value::i8(1)]).as_strings(),
            Some(vec![String::from("test1"), String::from("1")])
        );
        assert_eq!(Value::i8(1).as_strings(), None);
    }

    #[test]
    fn as_path_bufs() {
        assert_eq!(
            Value::Vec(vec![
                Value::PathBuf(PathBuf::from("test1")),
                Value::PathBuf(PathBuf::from("test2"))
            ])
            .as_path_bufs(),
            Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
        );
        assert_eq!(
            Value::Vec(vec![
                Value::PathBuf(PathBuf::from("test1")),
                Value::String(String::from("test2"))
            ])
            .as_path_bufs(),
            Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
        );
        assert_eq!(
            Value::Vec(vec![Value::PathBuf(PathBuf::from("test1")), Value::i8(1)]).as_path_bufs(),
            None
        );
        assert_eq!(Value::i8(1).as_path_bufs(), None);
    }
}
