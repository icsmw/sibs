mod any;
mod convertor;
mod error;
use crate::{
    elements::Cmb,
    // cli::args::exertion::Output,
    inf::journal::Output,
};
use any::DebugAny;
pub use convertor::*;
pub use error::E;
use std::{
    any::{Any, TypeId},
    fmt,
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum AnyValue {
    Empty,
    Output(Output),
    Cmb(Cmb),
    BoolTuple((bool, bool)),
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
    Vec(Vec<AnyValue>),
}

impl AnyValue {
    pub fn empty() -> Self {
        Self::Empty
    }
    pub fn new<T>(value: T) -> Result<Self, E>
    where
        T: DebugAny,
    {
        let dbg = format!("{value:?}");
        let boxed: Box<dyn Any> = Box::new(value);
        boxed
            .downcast::<()>()
            .map(|_| Self::Empty)
            .or_else(|boxed| boxed.downcast::<Output>().map(|v| Self::Output(*v)))
            .or_else(|boxed| boxed.downcast::<Cmb>().map(|v| Self::Cmb(*v)))
            .or_else(|boxed| {
                boxed
                    .downcast::<(bool, bool)>()
                    .map(|v| Self::BoolTuple(*v))
            })
            .or_else(|boxed| boxed.downcast::<String>().map(|v| Self::String(*v)))
            .or_else(|boxed| {
                boxed
                    .downcast::<&str>()
                    .map(|v| Self::String(v.to_string()))
            })
            .or_else(|boxed| boxed.downcast::<i8>().map(|v| Self::i8(*v)))
            .or_else(|boxed| boxed.downcast::<i16>().map(|v| Self::i16(*v)))
            .or_else(|boxed| boxed.downcast::<i32>().map(|v| Self::i32(*v)))
            .or_else(|boxed| boxed.downcast::<i64>().map(|v| Self::i64(*v)))
            .or_else(|boxed| boxed.downcast::<i128>().map(|v| Self::i128(*v)))
            .or_else(|boxed| boxed.downcast::<isize>().map(|v| Self::isize(*v)))
            .or_else(|boxed| boxed.downcast::<u8>().map(|v| Self::u8(*v)))
            .or_else(|boxed| boxed.downcast::<u16>().map(|v| Self::u16(*v)))
            .or_else(|boxed| boxed.downcast::<u32>().map(|v| Self::u32(*v)))
            .or_else(|boxed| boxed.downcast::<u64>().map(|v| Self::u64(*v)))
            .or_else(|boxed| boxed.downcast::<u128>().map(|v| Self::u128(*v)))
            .or_else(|boxed| boxed.downcast::<usize>().map(|v| Self::usize(*v)))
            .or_else(|boxed| boxed.downcast::<bool>().map(|v| Self::bool(*v)))
            .or_else(|boxed| boxed.downcast::<PathBuf>().map(|v| Self::PathBuf(*v)))
            .or_else(|boxed| {
                boxed
                    .downcast::<&String>()
                    .map(|v| Self::String((*v).clone()))
            })
            .or_else(|boxed| boxed.downcast::<&i8>().map(|v| Self::i8(**v)))
            .or_else(|boxed| boxed.downcast::<&i16>().map(|v| Self::i16(**v)))
            .or_else(|boxed| boxed.downcast::<&i32>().map(|v| Self::i32(**v)))
            .or_else(|boxed| boxed.downcast::<&i64>().map(|v| Self::i64(**v)))
            .or_else(|boxed| boxed.downcast::<&i128>().map(|v| Self::i128(**v)))
            .or_else(|boxed| boxed.downcast::<&isize>().map(|v| Self::isize(**v)))
            .or_else(|boxed| boxed.downcast::<&u8>().map(|v| Self::u8(**v)))
            .or_else(|boxed| boxed.downcast::<&u16>().map(|v| Self::u16(**v)))
            .or_else(|boxed| boxed.downcast::<&u32>().map(|v| Self::u32(**v)))
            .or_else(|boxed| boxed.downcast::<&u64>().map(|v| Self::u64(**v)))
            .or_else(|boxed| boxed.downcast::<&u128>().map(|v| Self::u128(**v)))
            .or_else(|boxed| boxed.downcast::<&usize>().map(|v| Self::usize(**v)))
            .or_else(|boxed| boxed.downcast::<&bool>().map(|v| Self::bool(**v)))
            .or_else(|boxed| {
                boxed
                    .downcast::<&PathBuf>()
                    .map(|v| Self::PathBuf((*v).clone()))
            })
            .or_else(|boxed| boxed.downcast::<Vec<AnyValue>>().map(|v| Self::Vec(*v)))
            .or(Err(E::NotSupportedType(dbg)))
    }

    pub fn duplicate(&self) -> Self {
        self.clone()
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self {
            AnyValue::Output(v) if TypeId::of::<T>() == TypeId::of::<Output>() => {
                Some(unsafe { &*(v as *const Output as *const T) })
            }
            AnyValue::Cmb(v) if TypeId::of::<T>() == TypeId::of::<Cmb>() => {
                Some(unsafe { &*(v as *const Cmb as *const T) })
            }
            AnyValue::BoolTuple(v) if TypeId::of::<T>() == TypeId::of::<(bool, bool)>() => {
                Some(unsafe { &*(v as *const (bool, bool) as *const T) })
            }
            AnyValue::i8(v) if TypeId::of::<T>() == TypeId::of::<i8>() => {
                Some(unsafe { &*(v as *const i8 as *const T) })
            }
            AnyValue::i16(v) if TypeId::of::<T>() == TypeId::of::<i16>() => {
                Some(unsafe { &*(v as *const i16 as *const T) })
            }
            AnyValue::i32(v) if TypeId::of::<T>() == TypeId::of::<i32>() => {
                Some(unsafe { &*(v as *const i32 as *const T) })
            }
            AnyValue::i64(v) if TypeId::of::<T>() == TypeId::of::<i64>() => {
                Some(unsafe { &*(v as *const i64 as *const T) })
            }
            AnyValue::i128(v) if TypeId::of::<T>() == TypeId::of::<i128>() => {
                Some(unsafe { &*(v as *const i128 as *const T) })
            }
            AnyValue::isize(v) if TypeId::of::<T>() == TypeId::of::<isize>() => {
                Some(unsafe { &*(v as *const isize as *const T) })
            }
            AnyValue::u8(v) if TypeId::of::<T>() == TypeId::of::<u8>() => {
                Some(unsafe { &*(v as *const u8 as *const T) })
            }
            AnyValue::u16(v) if TypeId::of::<T>() == TypeId::of::<u16>() => {
                Some(unsafe { &*(v as *const u16 as *const T) })
            }
            AnyValue::u32(v) if TypeId::of::<T>() == TypeId::of::<u32>() => {
                Some(unsafe { &*(v as *const u32 as *const T) })
            }
            AnyValue::u64(v) if TypeId::of::<T>() == TypeId::of::<u64>() => {
                Some(unsafe { &*(v as *const u64 as *const T) })
            }
            AnyValue::u128(v) if TypeId::of::<T>() == TypeId::of::<u128>() => {
                Some(unsafe { &*(v as *const u128 as *const T) })
            }
            AnyValue::usize(v) if TypeId::of::<T>() == TypeId::of::<usize>() => {
                Some(unsafe { &*(v as *const usize as *const T) })
            }
            AnyValue::bool(v) if TypeId::of::<T>() == TypeId::of::<bool>() => {
                Some(unsafe { &*(v as *const bool as *const T) })
            }
            AnyValue::PathBuf(v) if TypeId::of::<T>() == TypeId::of::<PathBuf>() => {
                Some(unsafe { &*(v as *const PathBuf as *const T) })
            }
            AnyValue::String(v) if TypeId::of::<T>() == TypeId::of::<String>() => {
                Some(unsafe { &*(v as *const String as *const T) })
            }
            AnyValue::Vec(v) if TypeId::of::<T>() == TypeId::of::<Vec<AnyValue>>() => {
                Some(unsafe { &*(v as *const Vec<AnyValue> as *const T) })
            }
            _ => None,
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self {
            AnyValue::Cmb(v) if TypeId::of::<T>() == TypeId::of::<Cmb>() => {
                Some(unsafe { &mut *(v as *mut Cmb as *mut T) })
            }
            AnyValue::BoolTuple(v) if TypeId::of::<T>() == TypeId::of::<(bool, bool)>() => {
                Some(unsafe { &mut *(v as *mut (bool, bool) as *mut T) })
            }
            AnyValue::i8(v) if TypeId::of::<T>() == TypeId::of::<i8>() => {
                Some(unsafe { &mut *(v as *mut i8 as *mut T) })
            }
            AnyValue::i16(v) if TypeId::of::<T>() == TypeId::of::<i16>() => {
                Some(unsafe { &mut *(v as *mut i16 as *mut T) })
            }
            AnyValue::i32(v) if TypeId::of::<T>() == TypeId::of::<i32>() => {
                Some(unsafe { &mut *(v as *mut i32 as *mut T) })
            }
            AnyValue::i64(v) if TypeId::of::<T>() == TypeId::of::<i64>() => {
                Some(unsafe { &mut *(v as *mut i64 as *mut T) })
            }
            AnyValue::i128(v) if TypeId::of::<T>() == TypeId::of::<i128>() => {
                Some(unsafe { &mut *(v as *mut i128 as *mut T) })
            }
            AnyValue::isize(v) if TypeId::of::<T>() == TypeId::of::<isize>() => {
                Some(unsafe { &mut *(v as *mut isize as *mut T) })
            }
            AnyValue::u8(v) if TypeId::of::<T>() == TypeId::of::<u8>() => {
                Some(unsafe { &mut *(v as *mut u8 as *mut T) })
            }
            AnyValue::u16(v) if TypeId::of::<T>() == TypeId::of::<u16>() => {
                Some(unsafe { &mut *(v as *mut u16 as *mut T) })
            }
            AnyValue::u32(v) if TypeId::of::<T>() == TypeId::of::<u32>() => {
                Some(unsafe { &mut *(v as *mut u32 as *mut T) })
            }
            AnyValue::u64(v) if TypeId::of::<T>() == TypeId::of::<u64>() => {
                Some(unsafe { &mut *(v as *mut u64 as *mut T) })
            }
            AnyValue::u128(v) if TypeId::of::<T>() == TypeId::of::<u128>() => {
                Some(unsafe { &mut *(v as *mut u128 as *mut T) })
            }
            AnyValue::usize(v) if TypeId::of::<T>() == TypeId::of::<usize>() => {
                Some(unsafe { &mut *(v as *mut usize as *mut T) })
            }
            AnyValue::bool(v) if TypeId::of::<T>() == TypeId::of::<bool>() => {
                Some(unsafe { &mut *(v as *mut bool as *mut T) })
            }
            AnyValue::PathBuf(v) if TypeId::of::<T>() == TypeId::of::<PathBuf>() => {
                Some(unsafe { &mut *(v as *mut PathBuf as *mut T) })
            }
            AnyValue::String(v) if TypeId::of::<T>() == TypeId::of::<String>() => {
                Some(unsafe { &mut *(v as *mut String as *mut T) })
            }
            AnyValue::Vec(v) if TypeId::of::<T>() == TypeId::of::<Vec<AnyValue>>() => {
                Some(unsafe { &mut *(v as *mut Vec<AnyValue> as *mut T) })
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
            Self::PathBuf(v) => Some(v.to_string_lossy().to_string()),
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

impl fmt::Display for AnyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create() {
        assert_eq!(AnyValue::new(1i8).unwrap(), AnyValue::i8(1));
        assert_eq!(AnyValue::new(1i16).unwrap(), AnyValue::i16(1));
        assert_eq!(AnyValue::new(1i32).unwrap(), AnyValue::i32(1));
        assert_eq!(AnyValue::new(1i64).unwrap(), AnyValue::i64(1));
        assert_eq!(AnyValue::new(1i128).unwrap(), AnyValue::i128(1));
        assert_eq!(AnyValue::new(1isize).unwrap(), AnyValue::isize(1));
        assert_eq!(AnyValue::new(1u8).unwrap(), AnyValue::u8(1));
        assert_eq!(AnyValue::new(1u16).unwrap(), AnyValue::u16(1));
        assert_eq!(AnyValue::new(1u32).unwrap(), AnyValue::u32(1));
        assert_eq!(AnyValue::new(1u64).unwrap(), AnyValue::u64(1));
        assert_eq!(AnyValue::new(1u128).unwrap(), AnyValue::u128(1));
        assert_eq!(AnyValue::new(1usize).unwrap(), AnyValue::usize(1));
        assert_eq!(AnyValue::new(true).unwrap(), AnyValue::bool(true));
        assert_eq!(
            AnyValue::new(PathBuf::from("test")).unwrap(),
            AnyValue::PathBuf(PathBuf::from("test"))
        );
        assert_eq!(
            AnyValue::new(String::from("test")).unwrap(),
            AnyValue::String(String::from("test"))
        );
        assert_eq!(
            AnyValue::new(vec![AnyValue::i8(1)]).unwrap(),
            AnyValue::Vec(vec![AnyValue::i8(1)])
        );
    }

    #[test]
    fn get() {
        assert_eq!(AnyValue::new(1i8).unwrap().get::<i8>(), Some(&1));
        assert_eq!(AnyValue::new(1i16).unwrap().get::<i16>(), Some(&1));
        assert_eq!(AnyValue::new(1i32).unwrap().get::<i32>(), Some(&1));
        assert_eq!(AnyValue::new(1i64).unwrap().get::<i64>(), Some(&1));
        assert_eq!(AnyValue::new(1i128).unwrap().get::<i128>(), Some(&1));
        assert_eq!(AnyValue::new(1isize).unwrap().get::<isize>(), Some(&1));
        assert_eq!(AnyValue::new(1u8).unwrap().get::<u8>(), Some(&1));
        assert_eq!(AnyValue::new(1u16).unwrap().get::<u16>(), Some(&1));
        assert_eq!(AnyValue::new(1u32).unwrap().get::<u32>(), Some(&1));
        assert_eq!(AnyValue::new(1u64).unwrap().get::<u64>(), Some(&1));
        assert_eq!(AnyValue::new(1u128).unwrap().get::<u128>(), Some(&1));
        assert_eq!(AnyValue::new(1usize).unwrap().get::<usize>(), Some(&1));
        assert_eq!(AnyValue::new(true).unwrap().get::<bool>(), Some(&true));
        assert_eq!(
            AnyValue::new(PathBuf::from("test"))
                .unwrap()
                .get::<PathBuf>(),
            Some(&PathBuf::from("test"))
        );
        assert_eq!(
            AnyValue::new(String::from("test")).unwrap().get::<String>(),
            Some(&String::from("test"))
        );
        assert_eq!(
            AnyValue::new(vec![AnyValue::i8(1)])
                .unwrap()
                .get::<Vec<AnyValue>>(),
            Some(&vec![AnyValue::i8(1)])
        );
    }

    #[test]
    fn get_mut() {
        let mut value = AnyValue::new(1i8).unwrap();
        if let Some(v) = value.get_mut::<i8>() {
            *v = 2;
        }
        assert_eq!(value.get::<i8>(), Some(&2));

        let mut value = AnyValue::new(1i16).unwrap();
        if let Some(v) = value.get_mut::<i16>() {
            *v = 2;
        }
        assert_eq!(value.get::<i16>(), Some(&2));

        let mut value = AnyValue::new(1i32).unwrap();
        if let Some(v) = value.get_mut::<i32>() {
            *v = 2;
        }
        assert_eq!(value.get::<i32>(), Some(&2));

        let mut value = AnyValue::new(1i64).unwrap();
        if let Some(v) = value.get_mut::<i64>() {
            *v = 2;
        }
        assert_eq!(value.get::<i64>(), Some(&2));

        let mut value = AnyValue::new(1i128).unwrap();
        if let Some(v) = value.get_mut::<i128>() {
            *v = 2;
        }
        assert_eq!(value.get::<i128>(), Some(&2));

        let mut value = AnyValue::new(1isize).unwrap();
        if let Some(v) = value.get_mut::<isize>() {
            *v = 2;
        }
        assert_eq!(value.get::<isize>(), Some(&2));

        let mut value = AnyValue::new(1u8).unwrap();
        if let Some(v) = value.get_mut::<u8>() {
            *v = 2;
        }
        assert_eq!(value.get::<u8>(), Some(&2));

        let mut value = AnyValue::new(1u16).unwrap();
        if let Some(v) = value.get_mut::<u16>() {
            *v = 2;
        }
        assert_eq!(value.get::<u16>(), Some(&2));

        let mut value = AnyValue::new(1u32).unwrap();
        if let Some(v) = value.get_mut::<u32>() {
            *v = 2;
        }
        assert_eq!(value.get::<u32>(), Some(&2));

        let mut value = AnyValue::new(1u64).unwrap();
        if let Some(v) = value.get_mut::<u64>() {
            *v = 2;
        }
        assert_eq!(value.get::<u64>(), Some(&2));

        let mut value = AnyValue::new(1u128).unwrap();
        if let Some(v) = value.get_mut::<u128>() {
            *v = 2;
        }
        assert_eq!(value.get::<u128>(), Some(&2));

        let mut value = AnyValue::new(1usize).unwrap();
        if let Some(v) = value.get_mut::<usize>() {
            *v = 2;
        }
        assert_eq!(value.get::<usize>(), Some(&2));

        let mut value = AnyValue::new(true).unwrap();
        if let Some(v) = value.get_mut::<bool>() {
            *v = false;
        }
        assert_eq!(value.get::<bool>(), Some(&false));

        let mut value = AnyValue::new(PathBuf::from("test")).unwrap();
        if let Some(v) = value.get_mut::<PathBuf>() {
            *v = PathBuf::from("modified");
        }
        assert_eq!(value.get::<PathBuf>(), Some(&PathBuf::from("modified")));

        let mut value = AnyValue::new(String::from("test")).unwrap();
        if let Some(v) = value.get_mut::<String>() {
            v.push_str(" modified");
        }
        assert_eq!(value.get::<String>(), Some(&String::from("test modified")));

        let mut value = AnyValue::new(vec![AnyValue::i8(1)]).unwrap();
        if let Some(v) = value.get_mut::<Vec<AnyValue>>() {
            v.push(AnyValue::i8(2));
        }
        assert_eq!(
            value.get::<Vec<AnyValue>>(),
            Some(&vec![AnyValue::i8(1), AnyValue::i8(2)])
        );
    }

    #[test]
    fn as_num() {
        assert_eq!(AnyValue::i8(1).as_num(), Some(1));
        assert_eq!(AnyValue::i16(1).as_num(), Some(1));
        assert_eq!(AnyValue::i32(1).as_num(), Some(1));
        assert_eq!(AnyValue::i64(1).as_num(), Some(1));
        assert_eq!(AnyValue::i128(1).as_num(), Some(1));
        assert_eq!(AnyValue::isize(1).as_num(), Some(1));
        assert_eq!(AnyValue::u8(1).as_num(), Some(1));
        assert_eq!(AnyValue::u16(1).as_num(), Some(1));
        assert_eq!(AnyValue::u32(1).as_num(), Some(1));
        assert_eq!(AnyValue::u64(1).as_num(), Some(1));
        assert_eq!(AnyValue::u128(1).as_num(), Some(1));
        assert_eq!(AnyValue::usize(1).as_num(), Some(1));
        assert_eq!(AnyValue::String(String::from("123")).as_num(), Some(123));
        assert_eq!(AnyValue::String(String::from("abc")).as_num(), None);
        assert_eq!(AnyValue::bool(true).as_num(), None);
        assert_eq!(AnyValue::PathBuf(PathBuf::from("test")).as_num(), None);
        assert_eq!(AnyValue::Vec(vec![AnyValue::i8(1)]).as_num(), None);
    }

    #[test]
    fn as_string() {
        assert_eq!(AnyValue::i8(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::i16(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::i32(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::i64(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::i128(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::isize(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::u8(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::u16(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::u32(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::u64(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::u128(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::usize(1).as_string(), Some("1".to_string()));
        assert_eq!(AnyValue::bool(true).as_string(), Some("true".to_string()));
        assert_eq!(
            AnyValue::PathBuf(PathBuf::from("test")).as_string(),
            Some("test".to_string())
        );
        assert_eq!(
            AnyValue::String(String::from("test")).as_string(),
            Some("test".to_string())
        );
        assert_eq!(AnyValue::Vec(vec![AnyValue::i8(1)]).as_string(), None);
    }

    #[test]
    fn as_bool() {
        // Testing integer types
        assert_eq!(AnyValue::i8(1).as_bool(), Some(true));
        assert_eq!(AnyValue::i8(0).as_bool(), Some(false));
        assert_eq!(AnyValue::i16(1).as_bool(), Some(true));
        assert_eq!(AnyValue::i16(0).as_bool(), Some(false));
        assert_eq!(AnyValue::i32(1).as_bool(), Some(true));
        assert_eq!(AnyValue::i32(0).as_bool(), Some(false));
        assert_eq!(AnyValue::i64(1).as_bool(), Some(true));
        assert_eq!(AnyValue::i64(0).as_bool(), Some(false));
        assert_eq!(AnyValue::i128(1).as_bool(), Some(true));
        assert_eq!(AnyValue::i128(0).as_bool(), Some(false));
        assert_eq!(AnyValue::isize(1).as_bool(), Some(true));
        assert_eq!(AnyValue::isize(0).as_bool(), Some(false));
        assert_eq!(AnyValue::u8(1).as_bool(), Some(true));
        assert_eq!(AnyValue::u8(0).as_bool(), Some(false));
        assert_eq!(AnyValue::u16(1).as_bool(), Some(true));
        assert_eq!(AnyValue::u16(0).as_bool(), Some(false));
        assert_eq!(AnyValue::u32(1).as_bool(), Some(true));
        assert_eq!(AnyValue::u32(0).as_bool(), Some(false));
        assert_eq!(AnyValue::u64(1).as_bool(), Some(true));
        assert_eq!(AnyValue::u64(0).as_bool(), Some(false));
        assert_eq!(AnyValue::u128(1).as_bool(), Some(true));
        assert_eq!(AnyValue::u128(0).as_bool(), Some(false));
        assert_eq!(AnyValue::usize(1).as_bool(), Some(true));
        assert_eq!(AnyValue::usize(0).as_bool(), Some(false));

        // Testing bool type
        assert_eq!(AnyValue::bool(true).as_bool(), Some(true));
        assert_eq!(AnyValue::bool(false).as_bool(), Some(false));

        // Testing String type
        assert_eq!(AnyValue::String(String::from("true")).as_bool(), Some(true));
        assert_eq!(
            AnyValue::String(String::from("false")).as_bool(),
            Some(false)
        );
        assert_eq!(AnyValue::String(String::from("TRUE")).as_bool(), Some(true));
        assert_eq!(
            AnyValue::String(String::from("FALSE")).as_bool(),
            Some(false)
        );
        assert_eq!(AnyValue::String(String::from("TrUe")).as_bool(), Some(true));
        assert_eq!(
            AnyValue::String(String::from("FaLsE")).as_bool(),
            Some(false)
        );
        assert_eq!(AnyValue::String(String::from("yes")).as_bool(), Some(false));
        assert_eq!(AnyValue::String(String::from("no")).as_bool(), Some(false));

        // Testing other types should return None
        assert_eq!(AnyValue::PathBuf(PathBuf::from("test")).as_bool(), None);
        assert_eq!(AnyValue::Vec(vec![AnyValue::i8(1)]).as_bool(), None);
    }

    #[test]
    fn as_path_buf() {
        assert_eq!(
            AnyValue::PathBuf(PathBuf::from("test")).as_path_buf(),
            Some(PathBuf::from("test"))
        );
        assert_eq!(
            AnyValue::String(String::from("test")).as_path_buf(),
            Some(PathBuf::from("test"))
        );
        assert_eq!(AnyValue::i8(1).as_path_buf(), None);
    }

    #[test]
    fn as_strings() {
        assert_eq!(
            AnyValue::Vec(vec![
                AnyValue::String(String::from("test1")),
                AnyValue::String(String::from("test2"))
            ])
            .as_strings(),
            Some(vec![String::from("test1"), String::from("test2")])
        );
        assert_eq!(
            AnyValue::Vec(vec![
                AnyValue::String(String::from("test1")),
                AnyValue::i8(1)
            ])
            .as_strings(),
            Some(vec![String::from("test1"), String::from("1")])
        );
        assert_eq!(AnyValue::i8(1).as_strings(), None);
    }

    #[test]
    fn as_path_bufs() {
        assert_eq!(
            AnyValue::Vec(vec![
                AnyValue::PathBuf(PathBuf::from("test1")),
                AnyValue::PathBuf(PathBuf::from("test2"))
            ])
            .as_path_bufs(),
            Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
        );
        assert_eq!(
            AnyValue::Vec(vec![
                AnyValue::PathBuf(PathBuf::from("test1")),
                AnyValue::String(String::from("test2"))
            ])
            .as_path_bufs(),
            Some(vec![PathBuf::from("test1"), PathBuf::from("test2")])
        );
        assert_eq!(
            AnyValue::Vec(vec![
                AnyValue::PathBuf(PathBuf::from("test1")),
                AnyValue::i8(1)
            ])
            .as_path_bufs(),
            None
        );
        assert_eq!(AnyValue::i8(1).as_path_bufs(), None);
    }
}
