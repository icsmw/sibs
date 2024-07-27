mod usecase;

use crate::reader::chars;
use std::process::Output;
use tokio::runtime::{Builder, Runtime};
pub use usecase::*;

pub const MAX_DEEP: usize = 5;

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

pub fn print_stdout(output: &Output) {
    let border = "=".repeat(100);
    println!(
        "{border}\n{}\n{border}\n{}{border}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
    );
}

pub fn get_rt() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .expect("Create tokio runtime")
}
