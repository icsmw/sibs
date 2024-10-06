mod debug;
mod err;
mod info;
mod verb;
mod warn;

use crate::{
    functions::{ExecutorFnDescription, E},
    inf::{Store, ValueRef},
};

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    store.insert(
        debug::name(),
        ExecutorFnDescription::new(debug::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    store.insert(
        err::name(),
        ExecutorFnDescription::new(err::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    store.insert(
        warn::name(),
        ExecutorFnDescription::new(warn::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    store.insert(
        info::name(),
        ExecutorFnDescription::new(info::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    store.insert(
        verb::name(),
        ExecutorFnDescription::new(verb::execute, vec![ValueRef::String], ValueRef::Empty),
    )?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        elements::{ElementRef, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };
    use std::{env::temp_dir, fs::read_to_string};
    use uuid::Uuid;

    const TESTS: &[&str] = &[
        r#"logs::err("Hello World!");"#,
        r#"logs::warn("Hello World!");"#,
        r#"logs::debug("Hello World!");"#,
        r#"logs::verb("Hello World!");"#,
        r#"logs::info("Hello World!");"#,
    ];

    const LOGS: &[&str] = &[
        "[ERROR]: Hello World!",
        "[WARNING]: Hello World!",
        "[DEBUG]: Hello World!",
        "[VERBOSE]: Hello World!",
        "[INFO]: Hello World!",
    ];

    #[tokio::test]
    async fn reading() {
        for (i, test) in TESTS.iter().enumerate() {
            let logs = temp_dir().join(format!("{}.log", Uuid::new_v4()));
            process_string!(
                &Configuration::to_file(logs.clone()),
                &format!("@test{{{test}}}"),
                |reader: &mut Reader, src: &mut Sources| {
                    let mut tasks: Vec<Element> = Vec::new();
                    while let Some(task) =
                        src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                    {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Element>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Element>, cx: Context, sc: Scope, journal: Journal| async move {
                    assert!(!tasks.is_empty());
                    for task in tasks.iter() {
                        let result = task
                            .execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                            .await;
                        if let Err(err) = result.as_ref() {
                            journal.report(err.into());
                        }
                        let logs = read_to_string(&logs).expect("Logs read");
                        journal.flush().await;
                        assert!(logs.contains(LOGS[i]));
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        }
    }
}
