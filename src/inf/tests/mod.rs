mod scope;
pub use scope::*;

use crate::reader::chars;
use tokio::runtime::{Builder, Runtime};

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

pub fn get_rt() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .expect("Create tokio runtime")
}
