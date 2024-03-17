use crate::{error::LinkedErr, inf::context::Context, reader::chars};
use tokio::runtime::{Builder, Runtime};

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

pub fn report_if_err<T, E: std::fmt::Display>(
    cx: &Context,
    result: Result<T, LinkedErr<E>>,
) -> Result<T, LinkedErr<E>> {
    if let Err(err) = result.as_ref() {
        cx.gen_report_from_err(err).expect("Generate error report");
        cx.post_reports();
    }
    result
}
pub fn post_if_err<T, E>(cx: &Context, result: Result<T, E>) -> Result<T, E> {
    if result.is_err() {
        cx.post_reports();
    }
    result
}

pub fn get_rt() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .expect("Create tokio runtime")
}
