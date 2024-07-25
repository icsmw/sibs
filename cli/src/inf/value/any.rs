use std::{any::Any, fmt::Debug};

pub trait DebugAny: Any + Debug + Send + Sync {
    #[allow(unused)]
    fn as_any(&self) -> &dyn Any;
    #[allow(unused)]
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
