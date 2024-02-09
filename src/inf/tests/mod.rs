mod scope;
pub use scope::*;

pub fn trim(src: &str) -> String {
    src.split('\n')
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("")
}
// use proptest::{prelude::*, strategy::ValueTree, test_runner::TestRunner};

// pub fn extract<T>(strategy: impl Strategy<Value = T>) -> T {
//     strategy
//         .new_tree(&mut TestRunner::default())
//         .unwrap()
//         .current()
// }

// pub fn random_bool() -> impl Strategy<Value = bool> {
//     prop_oneof![Just(true), Just(false)]
// }
