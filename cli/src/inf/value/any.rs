use std::{any::Any, fmt::Debug};

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

fn some(d: Box<dyn Any>) {
    d.downcast::<i8>();
}

fn ssss<T: DebugAny>(d: T) {
    let t: Box<dyn Any> = Box::new(d);
    if let Ok(v) = t.downcast::<i8>() {
        let v = *v;
    }
}
