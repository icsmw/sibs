use std::fmt;

#[derive(Debug, Clone)]
pub struct SimpleString {
    pub value: String,
    pub token: usize,
}

impl fmt::Display for SimpleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value,)
    }
}

#[cfg(test)]
mod proptest {

    use crate::{entry::SimpleString, inf::tests::*};
    use proptest::prelude::*;

    impl Arbitrary for SimpleString {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::SimpleString);
            let boxed = "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|value| SimpleString { value, token: 0 })
                .boxed();
            scope.write().unwrap().exclude(Entity::SimpleString);
            boxed
        }
    }
}
