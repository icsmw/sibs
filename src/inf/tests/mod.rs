mod scope;
pub use scope::Scope;

use proptest::{prelude::*, strategy::ValueTree, test_runner::TestRunner};

pub fn extract<T>(strategy: impl Strategy<Value = T>) -> T {
    strategy
        .new_tree(&mut TestRunner::default())
        .unwrap()
        .current()
}

pub fn random_bool() -> impl Strategy<Value = bool> {
    prop_oneof![Just(true), Just(false)]
}
