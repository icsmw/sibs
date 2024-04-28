mod get_cwd;
mod set_cwd;

use crate::{executors::Store, executors::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    fn name(func: &str) -> String {
        format!("cx::{func}")
    }
    store.insert(name(set_cwd::NAME), set_cwd::execute)?;
    store.insert(name(get_cwd::NAME), get_cwd::execute)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Reading, Sources},
    };
    use std::{env::temp_dir, fs::read_to_string};
    use uuid::Uuid;

    const TESTS: &[&str] = &[
        r#"@logs::err("Hello World!");"#,
        r#"@logs::warn("Hello World!");"#,
        r#"@logs::debug("Hello World!");"#,
        r#"@logs::verb("Hello World!");"#,
        r#"@logs::info("Hello World!");"#,
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
                &format!("test[{test}]"),
                |reader: &mut Reader, src: &mut Sources| {
                    let mut tasks: Vec<Task> = Vec::new();
                    while let Some(task) = src.report_err_if(Task::read(reader))? {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Task>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Task>, cx: Context, sc: Scope, journal: Journal| async move {
                    for task in tasks.iter() {
                        let result = task.execute(None, &[], &[], cx.clone(), sc.clone()).await;
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
