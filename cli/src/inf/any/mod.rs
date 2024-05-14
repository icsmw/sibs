use crate::executors::{TryAnyTo, E};
use std::{
    any::{Any, TypeId},
    fmt,
    fmt::Debug,
    path::PathBuf,
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
enum InnerType {
    Multiple,
    Single,
}

#[derive(Debug)]
pub struct AnyValue {
    value: Box<dyn DebugAny>,
    values: Vec<AnyValue>,
    inner_ty: InnerType,
    type_id: TypeId,
}

impl AnyValue {
    pub fn new<T>(val: T) -> Self
    where
        T: DebugAny,
    {
        Self {
            value: Box::new(val),
            values: Vec::new(),
            inner_ty: InnerType::Single,
            type_id: TypeId::of::<T>(),
        }
    }

    pub fn vec(values: Vec<AnyValue>) -> Self {
        Self {
            value: Box::new(()),
            values,
            inner_ty: InnerType::Multiple,
            type_id: TypeId::of::<()>(),
        }
    }

    pub fn get_as<T: 'static>(&self) -> Option<&T> {
        if TypeId::of::<T>() == self.type_id {
            self.value.as_ref().as_any().downcast_ref()
        } else {
            None
        }
    }

    pub fn duplicate(&self) -> Result<AnyValue, E> {
        match self.inner_ty {
            InnerType::Single => {
                if let Some(v) = self.get_as::<AnyValue>() {
                    return v.duplicate();
                } else if let Some(v) = self.get_as::<Vec<AnyValue>>() {
                    let mut out: Vec<AnyValue> = Vec::new();
                    for value in v.iter() {
                        out.push(value.duplicate()?);
                    }
                    return Ok(AnyValue::new(out));
                }
                self.
                // Check primitives
                get_as::<bool>()
                .map(|v| AnyValue::new(v.to_owned()))
                .or_else(|| self.get_as::<i8>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<i16>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<i32>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<i64>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<i128>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<isize>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<u8>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<u16>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<u32>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<u64>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<u128>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<usize>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<f32>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<f64>().map(|v| AnyValue::new(*v)))
                .or_else(|| self.get_as::<String>().map(|v| AnyValue::new(v.to_owned())))
                .or_else(|| {
                    self.get_as::<PathBuf>()
                        .map(|v| AnyValue::new(v.to_owned()))
                })
                // Check vectors
                .or_else(|| {
                    self.get_as::<Vec<bool>>()
                        .map(|v| AnyValue::new::<Vec<bool>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<i8>>()
                        .map(|v| AnyValue::new::<Vec<i8>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<i16>>()
                        .map(|v| AnyValue::new::<Vec<i16>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<i32>>()
                        .map(|v| AnyValue::new::<Vec<i32>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<i64>>()
                        .map(|v| AnyValue::new::<Vec<i64>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<i128>>()
                        .map(|v| AnyValue::new::<Vec<i128>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<isize>>()
                        .map(|v| AnyValue::new::<Vec<isize>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<u8>>()
                        .map(|v| AnyValue::new::<Vec<u8>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<u16>>()
                        .map(|v| AnyValue::new::<Vec<u16>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<u32>>()
                        .map(|v| AnyValue::new::<Vec<u32>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<u64>>()
                        .map(|v| AnyValue::new::<Vec<u64>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<u128>>()
                        .map(|v| AnyValue::new::<Vec<u128>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<usize>>()
                        .map(|v| AnyValue::new::<Vec<usize>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<f32>>()
                        .map(|v| AnyValue::new::<Vec<f32>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<f64>>()
                        .map(|v| AnyValue::new::<Vec<f64>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<String>>()
                        .map(|v| AnyValue::new::<Vec<String>>(v.clone()))
                })
                .or_else(|| {
                    self.get_as::<Vec<PathBuf>>()
                        .map(|v| AnyValue::new::<Vec<PathBuf>>(v.clone()))
                }).ok_or(E::NotSupportedType(format!("Value: {:?}; type: {:?}", self.value, self.type_id)))
            }
            InnerType::Multiple => {
                let mut values: Vec<AnyValue> = Vec::new();
                for value in self.values.iter() {
                    values.push(value.duplicate()?);
                }
                Ok(AnyValue::vec(values))
            }
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
                    .downcast_ref::<PathBuf>()
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
                reference.downcast_ref::<Vec<PathBuf>>().map(|v| {
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

    pub fn is_vec(&self) -> bool {
        matches!(self.inner_ty, InnerType::Multiple)
    }

    pub fn as_vec(&self) -> &[AnyValue] {
        &self.values
    }
}

impl fmt::Display for AnyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl TryAnyTo<PathBuf> for AnyValue {
    fn try_to(&self) -> Result<PathBuf, E> {
        Ok(PathBuf::from(
            self.get_as_string()
                .ok_or(E::Converting(String::from("PathBuf")))?,
        ))
    }
}

impl TryAnyTo<Vec<PathBuf>> for AnyValue {
    fn try_to(&self) -> Result<Vec<PathBuf>, E> {
        let vec = Ok(self
            .get_as_strings()
            .ok_or(E::Converting(String::from("PathBuf")))?
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>());
        vec
    }
}

impl TryAnyTo<String> for AnyValue {
    fn try_to(&self) -> Result<String, E> {
        Ok(self
            .get_as_string()
            .ok_or(E::Converting(String::from("String")))?
            .to_owned())
    }
}

impl TryAnyTo<usize> for AnyValue {
    fn try_to(&self) -> Result<usize, E> {
        usize::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("usize")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to usize")))
    }
}

impl TryAnyTo<u128> for AnyValue {
    fn try_to(&self) -> Result<u128, E> {
        u128::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("u128")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to u128")))
    }
}

impl TryAnyTo<u64> for AnyValue {
    fn try_to(&self) -> Result<u64, E> {
        u64::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("u64")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to u64")))
    }
}

impl TryAnyTo<u32> for AnyValue {
    fn try_to(&self) -> Result<u32, E> {
        u32::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("u32")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to u32")))
    }
}

impl TryAnyTo<u16> for AnyValue {
    fn try_to(&self) -> Result<u16, E> {
        u16::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("u16")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to u16")))
    }
}

impl TryAnyTo<u8> for AnyValue {
    fn try_to(&self) -> Result<u8, E> {
        u8::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("u8")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to u8")))
    }
}

impl TryAnyTo<isize> for AnyValue {
    fn try_to(&self) -> Result<isize, E> {
        self.get_as_integer()
            .ok_or(E::Converting(String::from("isize")))
    }
}

impl TryAnyTo<i128> for AnyValue {
    fn try_to(&self) -> Result<i128, E> {
        i128::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("i128")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to i128")))
    }
}

impl TryAnyTo<i64> for AnyValue {
    fn try_to(&self) -> Result<i64, E> {
        i64::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("i64")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to i64")))
    }
}

impl TryAnyTo<i32> for AnyValue {
    fn try_to(&self) -> Result<i32, E> {
        i32::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("i32")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to i32")))
    }
}

impl TryAnyTo<i16> for AnyValue {
    fn try_to(&self) -> Result<i16, E> {
        i16::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("i16")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to i16")))
    }
}

impl TryAnyTo<i8> for AnyValue {
    fn try_to(&self) -> Result<i8, E> {
        i8::try_from(
            self.get_as_integer()
                .ok_or(E::Converting(String::from("i8")))?,
        )
        .map_err(|_| E::Converting(String::from("isize to i8")))
    }
}

impl TryAnyTo<bool> for AnyValue {
    fn try_to(&self) -> Result<bool, E> {
        Ok(self
            .get_as_bool()
            .ok_or(E::Converting(String::from("bool")))?
            .to_owned())
    }
}
