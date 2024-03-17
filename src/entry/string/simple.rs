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

    use crate::entry::SimpleString;
    use proptest::prelude::*;

    impl Arbitrary for SimpleString {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|value| SimpleString {
                    value: if value.is_empty() {
                        "min".to_owned()
                    } else {
                        value
                    },
                    token: 0,
                })
                .boxed()
        }
    }
}
