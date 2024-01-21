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
}
