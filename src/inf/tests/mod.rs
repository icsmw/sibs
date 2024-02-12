mod scope;
pub use scope::*;

use crate::reader::chars;

pub fn trim_carets(src: &str) -> String {
    src.split('\n')
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("")
}
pub fn trim_semicolon(src: &str) -> String {
    if src.ends_with(chars::SEMICOLON) {
        src[0..src.len() - 1].to_owned()
    } else {
        src.to_owned()
    }
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
