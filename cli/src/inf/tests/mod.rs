use crate::{
    error::LinkedErr,
    inf::context::Context,
    reader::{chars, Reader, Sources},
};
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

pub fn report_reading_err<T, E: std::error::Error + std::fmt::Display + ToString>(
    src: &mut Sources,
    result: Result<T, LinkedErr<E>>,
) -> Result<T, LinkedErr<E>> {
    if let Err(err) = result.as_ref() {
        src.report_err(err).expect("Generate error report");
    }
    result
}

pub fn runner<T, E: std::error::Error + std::fmt::Display + ToString, F>(
    content: &str,
    cb: F,
) -> Result<T, LinkedErr<E>>
where
    F: FnOnce(Sources, Reader) -> Result<T, LinkedErr<E>>,
{
    let mut src = Sources::new();
    let reader = src
        .reader()
        .unbound(content)
        .expect("Unbound reader is created");
    cb(src, reader)
}

pub fn get_reader_for_str(content: &str) -> (Sources, Reader) {
    let mut src = Sources::new();
    let reader = src
        .reader()
        .unbound(content)
        .expect("Unbound reader is created");
    (src, reader)
}

pub fn get_rt() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .expect("Create tokio runtime")
}
