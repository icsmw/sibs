use std::{
    any::{Any, TypeId},
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
pub trait D: std::fmt::Display + Sized + 'static {}

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
            .and_then(|v| Some(v.to_owned()))
            .or_else(|| {
                reference
                    .downcast_ref::<usize>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u8>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u16>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u32>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<u64>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i8>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i16>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i32>()
                    .and_then(|v| Some(v.to_string()))
            })
            .or_else(|| {
                reference
                    .downcast_ref::<i64>()
                    .and_then(|v| Some(v.to_string()))
            })
    }
}
