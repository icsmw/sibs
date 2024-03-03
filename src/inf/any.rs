use crate::executors::{TryAnyTo, E};
use std::{
    any::{Any, TypeId},
    fmt,
    fmt::Debug,
};
pub trait DebugAny: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Debug + 'static> DebugAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug)]
pub struct AnyValue {
    value: Box<dyn DebugAny>,
    type_id: TypeId,
}

impl AnyValue {
    pub fn new<T>(val: T) -> Self
    where
        T: DebugAny,
    {
        Self {
            value: Box::new(val),
            type_id: TypeId::of::<T>(),
        }
    }

    pub fn get_as<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == self.type_id {
            self.value.as_ref().as_any().downcast_ref()
        } else {
            None
        }
    }

    pub fn get_as_string(&self) -> Option<String> {
        let reference = self.value.as_ref().as_any();
        reference
            .downcast_ref::<String>()
            .map(|v| v.to_owned())
            .or_else(|| reference.downcast_ref::<usize>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<u8>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<u16>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<u32>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<u64>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<i8>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<i16>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<i32>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<i64>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<bool>().map(|v| v.to_string()))
            .or_else(|| {
                reference
                    .downcast_ref::<std::path::PathBuf>()
                    .map(|v| v.to_string_lossy().to_string())
            })
    }

    pub fn get_as_strings(&self) -> Option<Vec<String>> {
        let reference = self.value.as_ref().as_any();
        reference
            .downcast_ref::<Vec<String>>()
            .map(|v| v.to_owned())
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<usize>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<u8>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<u16>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<u32>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<u64>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<i8>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<i16>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<i32>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<i64>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<bool>>()
                    .map(|v| v.iter().map(|v| v.to_string()).collect::<Vec<String>>())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<Vec<std::path::PathBuf>>()
                    .map(|v| {
                        v.iter()
                            .map(|v| v.to_string_lossy().to_string())
                            .collect::<Vec<String>>()
                    })
            })
    }
}

impl fmt::Display for AnyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl TryAnyTo<std::path::PathBuf> for AnyValue {
    fn try_to(&self) -> Result<std::path::PathBuf, E> {
        Ok(std::path::PathBuf::from(
            self.get_as_string()
                .ok_or(E::Converting(String::from("PathBuf")))?,
        ))
    }
}

impl TryAnyTo<String> for AnyValue {
    fn try_to(&self) -> Result<String, E> {
        Ok(self
            .get_as::<String>()
            .ok_or(E::Converting(String::from("String")))?
            .to_owned())
    }
}
