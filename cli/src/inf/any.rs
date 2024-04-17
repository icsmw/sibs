use crate::executors::{TryAnyTo, E};
use std::{
    any::{Any, TypeId},
    fmt,
    fmt::Debug,
};
pub trait DebugAny: Any + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Debug + Send + Sync + 'static> DebugAny for T {
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

    pub fn get_as_bool(&self) -> Option<bool> {
        let reference = self.value.as_ref().as_any();
        reference
            .downcast_ref::<bool>()
            .map(|v| v.to_owned())
            .or_else(|| reference.downcast_ref::<usize>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<isize>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<u8>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<u16>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<u32>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<u64>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<i8>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<i16>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<i32>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<i64>().map(|v| *v > 1))
            .or_else(|| reference.downcast_ref::<String>().map(|v| v == "true"))
            .or_else(|| {
                reference
                    .downcast_ref::<AnyValue>()
                    .and_then(|v| v.get_as_bool())
            })
    }

    pub fn get_as_integer(&self) -> Option<isize> {
        let reference = self.value.as_ref().as_any();
        reference
            .downcast_ref::<String>()
            .map(|v| v.parse::<isize>().ok())
            .or_else(|| {
                reference
                    .downcast_ref::<usize>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| reference.downcast_ref::<isize>().map(|v| Some(*v)))
            .or_else(|| {
                reference
                    .downcast_ref::<u128>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u64>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u32>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u16>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| reference.downcast_ref::<u8>().map(|v| Some(*v as isize)))
            .or_else(|| {
                reference
                    .downcast_ref::<i128>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i64>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i32>()
                    .map(|v| isize::try_from(*v).ok())
            })
            .or_else(|| reference.downcast_ref::<i16>().map(|v| Some(*v as isize)))
            .or_else(|| reference.downcast_ref::<i8>().map(|v| Some(*v as isize)))
            .or_else(|| {
                reference
                    .downcast_ref::<AnyValue>()
                    .map(|v| v.get_as_integer())
            })
            .unwrap_or(None)
    }

    pub fn get_as_string(&self) -> Option<String> {
        let reference = self.value.as_ref().as_any();
        reference
            .downcast_ref::<String>()
            .map(|v| v.to_owned())
            .or_else(|| reference.downcast_ref::<usize>().map(|v| v.to_string()))
            .or_else(|| reference.downcast_ref::<isize>().map(|v| v.to_string()))
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
            .or_else(|| {
                reference
                    .downcast_ref::<AnyValue>()
                    .and_then(|v| v.get_as_string())
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
                    .downcast_ref::<Vec<isize>>()
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
            .or_else(|| {
                let try_collect = |values: Option<&Vec<AnyValue>>| -> Result<Vec<String>, ()> {
                    let values = values.ok_or(())?;
                    let mut strings: Vec<String> = Vec::new();
                    for v in values.iter() {
                        strings.push(v.get_as_string().ok_or(())?);
                    }
                    Ok(strings)
                };
                match try_collect(reference.downcast_ref::<Vec<AnyValue>>()) {
                    Ok(v) => Some(v),
                    Err(_) => None,
                }
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

impl TryAnyTo<Vec<std::path::PathBuf>> for AnyValue {
    fn try_to(&self) -> Result<Vec<std::path::PathBuf>, E> {
        let vec = Ok(self
            .get_as_strings()
            .ok_or(E::Converting(String::from("PathBuf")))?
            .iter()
            .map(std::path::PathBuf::from)
            .collect::<Vec<std::path::PathBuf>>());
        vec
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
