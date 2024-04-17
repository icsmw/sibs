use crate::{
    error::LinkedErr,
    inf::{
        journal::{Configuration, Journal},
        Context, Scenario, Scope,
    },
    reader::{chars, Reader, Sources},
};
use std::{future::Future, path::PathBuf, pin::Pin};
use tokio::runtime::{Builder, Runtime};

use super::journal;

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

pub fn report_reading_err<T, E: Clone + std::error::Error + std::fmt::Display + ToString>(
    src: &mut Sources,
    result: Result<T, LinkedErr<E>>,
) -> Result<T, LinkedErr<E>> {
    if let Err(err) = result.as_ref() {
        src.report_err(err).expect("Generate error report");
    }
    result
}

pub async fn runner<T, E: Clone + std::error::Error + std::fmt::Display + ToString, F>(
    content: &str,
    cb: F,
) -> Result<T, LinkedErr<E>>
where
    F: FnOnce(Sources, Reader) -> Result<T, LinkedErr<E>>,
{
    let journal = Journal::init(Configuration::logs());
    let mut src = Sources::new(&journal);
    let reader = src
        .reader()
        .unbound(content)
        .expect("Unbound reader is created");
    let result = cb(src, reader);
    journal.destroy().await;
    result
}

pub type ExecutionResult<'a, R> = Pin<Box<dyn Future<Output = R> + 'a>>;

pub async fn execution<'a, T, E: Clone + std::error::Error + std::fmt::Display + ToString, F>(
    src: &Sources,
    cb: F,
) -> Result<T, LinkedErr<E>>
where
    F: FnOnce(Context, Scope) -> ExecutionResult<'a, Result<T, LinkedErr<E>>>,
{
    let journal = Journal::init(Configuration::logs());
    let cx = match Context::init(Scenario::dummy(), src, &journal) {
        Ok(cx) => cx,
        Err(err) => panic!("Fail to create executing context: {err}"),
    };
    let sc = Scope::init(Some(cx.scenario.filename.clone()));
    let result = cb(cx.clone(), sc.clone()).await;
    if let Err(err) = sc.destroy().await {
        eprint!("Fail to destroy executing scope: {err}");
    }
    if let Err(err) = cx.destroy().await {
        eprint!("Fail to destroy executing context: {err}");
    }
    if let Err(err) = journal.destroy().await {
        eprint!("Fail to destroy journal: {err}");
    }
    result
}

pub async fn execution_from_file<
    'a,
    T,
    E: Clone + std::error::Error + std::fmt::Display + ToString,
    F,
>(
    filename: &PathBuf,
    src: &Sources,
    cb: F,
) -> Result<T, LinkedErr<E>>
where
    F: FnOnce(Context, Scope) -> ExecutionResult<'a, Result<T, LinkedErr<E>>>,
{
    let journal = Journal::init(Configuration::logs());
    let cx = match Context::init(
        Scenario::from(filename).expect("correct scenario path"),
        src,
        &journal,
    ) {
        Ok(cx) => cx,
        Err(err) => panic!("Fail to create executing context: {err}"),
    };
    let sc = Scope::init(Some(cx.scenario.filename.clone()));
    let result = cb(cx.clone(), sc.clone()).await;
    if let Err(err) = sc.destroy().await {
        eprint!("Fail to destroy executing scope: {err}");
    }
    if let Err(err) = cx.destroy().await {
        eprint!("Fail to destroy executing context: {err}");
    }
    if let Err(err) = journal.destroy().await {
        eprint!("Fail to destroy journal: {err}");
    }
    result
}

pub fn get_reader_for_str(content: &str) -> (Sources, Reader, Journal) {
    let journal = Journal::init(Configuration::logs());
    let mut src = Sources::new(&journal);
    let reader = src
        .reader()
        .unbound(content)
        .expect("Unbound reader is created");
    (src, reader, journal)
}

pub fn get_rt() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("runtime")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .expect("Create tokio runtime")
}
