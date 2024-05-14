use crate::inf::AnyValue;
use std::any::{type_name_of_val, Any, TypeId};

struct Value<T> {
    value: T,
}

impl<T: 'static + Clone> Value<T> {
    pub fn new(v: &dyn Any) -> Self {
        Self {
            value: v.downcast_ref::<T>().unwrap().clone(),
        }
    }
}
impl Value<u8> {
    pub fn create(&self) -> AnyValue {
        AnyValue::new(self.value)
    }
    fn get(&self) -> &u8 {
        &self.value
    }
}

impl Value<u16> {
    pub fn create(&self) -> AnyValue {
        AnyValue::new(self.value)
    }
    fn get(&self) -> &u16 {
        &self.value
    }
}

impl Value<u32> {
    pub fn create(&self) -> AnyValue {
        AnyValue::new(self.value)
    }
    fn get(&self) -> &u32 {
        &self.value
    }
}

impl Value<f64> {
    pub fn create(&self) -> AnyValue {
        AnyValue::new(self.value)
    }
    fn get(&self) -> &f64 {
        &self.value
    }
}
impl Value<String> {
    pub fn create(&self) -> AnyValue {
        AnyValue::new(self.value.clone())
    }
    fn get(&self) -> &String {
        &self.value
    }
}
// impl Attribute<String> {}

fn is<T: 'static>(s: &dyn Any) -> bool {
    TypeId::of::<T>() == s.type_id()
}

#[macro_export]
macro_rules! any_value2 {
    ($val:expr) => {
        if type_name_of_val(&$val).contains("str") {
            panic!("Hey!");
        } else if is::<u8>(&$val) {
            AnyValue::new(Value::<u8>::new(&$val).get().clone())
        } else if is::<u16>(&$val) {
            AnyValue::new(Value::<u16>::new(&$val).get().clone())
        } else if is::<String>(&$val) {
            AnyValue::new(Value::<String>::new(&$val).get().clone())
        } else {
            panic!("I dunno what is `{:?}` :(", $val);
        }
    };
}

#[macro_export]
macro_rules! any_value {
    ( $e:expr ) => {
        AnyValue::new(Value { value: $e }.get().clone())
    };
}
#[test]
fn test() {
    Value::<u8>::new(&12).create();
    println!(
        "{:?}: {:?}",
        any_value2!(12u8),
        std::any::TypeId::of::<u8>()
    );
    println!("{:?}", any_value2!(12222u16));
    println!("{:?}", any_value2!(12u8));
    println!(
        "{:?}: {:?}",
        any_value2!("fdsfsdfsd".to_owned()),
        std::any::TypeId::of::<String>()
    );
    println!(
        "{:?}: {:?}",
        any_value2!("fdsfsdfsd"),
        std::any::TypeId::of::<&str>()
    );

    // println!("{:?}", any_value!("fdsfsdfsd"));
}
