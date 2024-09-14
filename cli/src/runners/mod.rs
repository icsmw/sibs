use crate::inf::{journal::Report, Journal};
use std::process;

#[allow(unused)]
pub async fn exit<T>(journal: Journal, err: T)
where
    T: std::fmt::Debug + Into<Report>,
{
    journal.report(err.into());
    if journal.destroy().await.is_err() {
        eprintln!("Fail destroy journal");
    }
    // TODO: implement error's codes
    process::exit(1);
}

#[cfg(test)]
pub async fn process_block<S: AsRef<str>, T>(block: S, expectation: T)
where
    T: 'static + Clone + PartialEq + std::fmt::Debug,
{
    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            journal::Journal,
            operator::{Execute, E},
            Configuration, Context, Scope, Value,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };
    use tokio_util::sync::CancellationToken;

    let task = format!("@test(){{{}}}", block.as_ref());
    process_string!(
        &Configuration::logs(false),
        &task,
        |reader: &mut Reader, src: &mut Sources| {
            let mut tasks: Vec<Element> = Vec::new();
            while let Some(task) = src.report_err_if(Element::include(reader, &[ElTarget::Task]))? {
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                tasks.push(task);
            }
            if tasks.is_empty() {
                eprintln!("Fail read task from:\n{task}\n");
            }
            assert!(!tasks.is_empty());
            Ok::<Vec<Element>, LinkedErr<E>>(tasks)
        },
        |tasks: Vec<Element>, cx: Context, sc: Scope, _journal: Journal| async move {
            for task in tasks.iter() {
                let result = task
                    .execute(
                        None,
                        &[],
                        &[],
                        &None,
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new(),
                    )
                    .await;

                if let Err(err) = result.as_ref() {
                    cx.atlas
                        .report_err(err)
                        .await
                        .expect("Error report has been created");
                }
                let result = result.expect("run of task is success");
                let expectation_as_any = Box::new(expectation.clone()) as Box<dyn Any>;
                if let Ok(expectation) = expectation_as_any.downcast::<Value>() {
                    assert_eq!(result, *expectation);
                } else {
                    assert_eq!(
                        result.get::<T>().expect("test returns correct value"),
                        &expectation
                    );
                }
            }
            Ok::<(), LinkedErr<E>>(())
        }
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! test_block {
    ($fn_name:ident, $content:literal, $expectation:expr) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_ $fn_name >] () {
                $crate::runners::process_block($content, $expectation).await;
            }
        }
    };
}

#[macro_export]
macro_rules! read_string {
    ($cfg:expr, $content:expr, $reading:expr) => {{
        use std::{
            any::Any,
            panic::{self, AssertUnwindSafe},
        };
        use $crate::{inf::Journal, runners::exit};

        let journal = Journal::unwrapped(Configuration::logs(false));
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting & Configuration as the first argument").await;
        };
        let content = $content as &dyn Any;
        let content = if let Some(content) = content.downcast_ref::<&str>().cloned() {
            content.to_string()
        } else if let Some(content) = content.downcast_ref::<String>().cloned() {
            content
        } else {
            return exit(journal, "Expecting &content as the second argument").await;
        };
        let journal = Journal::unwrapped(cfg);
        let mut src = Sources::new(&journal);
        let mut reader = Reader::unbound(&mut src, &content).expect("Unbound reader is created");
        #[allow(clippy::redundant_closure_call)]
        let result = panic::catch_unwind(AssertUnwindSafe(|| $reading(&mut reader, &mut src)));
        let output = match result {
            Err(e) => {
                return exit(journal, &format!("paniced with: {e:?}")).await;
            }
            Ok(Err(e)) => {
                return exit(journal, &e.to_string()).await;
            }
            Ok(Ok(output)) => output,
        };
        if journal.destroy().await.is_err() {
            eprintln!("Fail destroy journal");
        }
        output
    }};
}

#[macro_export]
macro_rules! process_string {
    ($cfg:expr, $content:expr, $reading:expr, $executing:expr) => {{
        use futures::FutureExt;
        use std::{
            any::Any,
            panic::{self, AssertUnwindSafe},
        };
        use $crate::{inf::Scenario, runners::exit};
        let journal = Journal::unwrapped(Configuration::logs(false));
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting &Configuration as the first argument").await;
        };
        let content = $content as &dyn Any;
        let content = if let Some(content) = content.downcast_ref::<&str>().cloned() {
            content.to_string()
        } else if let Some(content) = content.downcast_ref::<String>().cloned() {
            content
        } else {
            return exit(journal, "Expecting &content as the second argument").await;
        };
        let journal = Journal::unwrapped(cfg);
        let mut src = Sources::new(&journal);
        let mut reader = Reader::unbound(&mut src, &content).expect("Unbound reader is created");
        #[allow(clippy::redundant_closure_call)]
        let result = panic::catch_unwind(AssertUnwindSafe(|| $reading(&mut reader, &mut src)));
        let output = match result {
            Err(e) => {
                return exit(journal, &format!("paniced with: {e:?}")).await;
            }
            Ok(Err(e)) => {
                return exit(journal, &e.to_string()).await;
            }
            Ok(Ok(output)) => output,
        };
        let cx = Context::init(Scenario::dummy(), &src, &journal).expect("Context is created");
        let sc = cx
            .scope
            .create("TestRunner", Some(cx.scenario.path.clone()))
            .await
            .expect("Scope is created");
        #[allow(clippy::redundant_closure_call)]
        let result = AssertUnwindSafe($executing(output, cx.clone(), sc, journal.clone()))
            .catch_unwind()
            .await;
        let _ = cx.destroy().await;
        match result {
            Err(e) => {
                return exit(journal, &format!("{e:?}")).await;
            }
            Ok(Err(e)) => {
                return exit(journal, &e.to_string()).await;
            }
            _ => {}
        };
        if journal.destroy().await.is_err() {
            eprintln!("Fail destroy journal");
        }
    }};
}

#[macro_export]
macro_rules! process_file {
    ($cfg:expr, $filename:expr, $executing:expr) => {{
        use futures::FutureExt;
        use std::{any::Any, panic::AssertUnwindSafe, path::PathBuf};
        use $crate::{inf::Scenario, runners::exit};

        let journal = Journal::unwrapped(Configuration::logs(false));
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting &Configuration as the first argument").await;
        };
        let filename = $filename as &dyn Any;
        let Some(filename) = filename.downcast_ref::<PathBuf>().cloned() else {
            return exit(journal, "Expecting &PathBuf as the second argument").await;
        };
        let journal = Journal::unwrapped(cfg);
        let scenario = match Scenario::from(&filename) {
            Ok(scenario) => scenario,
            Err(err) => {
                return exit(journal, &err.to_string()).await;
            }
        };
        let mut src = Sources::new(&journal);
        let elements =
            match Reader::read_file(&scenario.filename, true, Some(&mut src), &journal).await {
                Ok(elements) => elements,
                Err(err) => {
                    if let Err(err) = src.report_err(&err) {
                        journal.report(err.to_string().into());
                    }
                    return exit(journal, &err).await;
                }
            };
        let cx = Context::init(scenario, &src, &journal).expect("Context is created");
        let sc = cx
            .scope
            .create("TestRunner", Some(cx.scenario.path.clone()))
            .await
            .expect("Scope is created");
        #[allow(clippy::redundant_closure_call)]
        let result = AssertUnwindSafe($executing(elements, cx.clone(), sc, journal.clone()))
            .catch_unwind()
            .await;
        let _ = cx.destroy().await;
        match result {
            Err(e) => {
                return exit(journal, &format!("{e:?}")).await;
            }
            Ok(Err(e)) => {
                return exit(journal, &e.to_string()).await;
            }
            _ => {}
        };
        if journal.destroy().await.is_err() {
            eprintln!("Fail destroy journal");
        }
    }};
}
#[macro_export]
macro_rules! read_file {
    ($cfg:expr, $filename:expr, $executing:expr) => {{
        use futures::FutureExt;
        use std::{any::Any, panic::AssertUnwindSafe, path::PathBuf};
        use $crate::{
            inf::{Context, Scenario},
            runners::exit,
        };

        let journal = Journal::unwrapped(Configuration::logs(false));
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting &Configuration as the first argument").await;
        };
        let filename = $filename as &dyn Any;
        let Some(filename) = filename.downcast_ref::<PathBuf>().cloned() else {
            return exit(journal, "Expecting &PathBuf as the second argument").await;
        };
        let journal = Journal::unwrapped(cfg);
        let scenario = match Scenario::from(&filename) {
            Ok(scenario) => scenario,
            Err(err) => {
                return exit(journal, &err.to_string()).await;
            }
        };
        let mut src = Sources::new(&journal);
        let elements =
            match Reader::read_file(&scenario.filename, true, Some(&mut src), &journal).await {
                Ok(elements) => elements,
                Err(err) => {
                    if let Err(err) = src.report_err(&err) {
                        journal.report(err.to_string().into());
                    }
                    return exit(journal, &err).await;
                }
            };
        let cx = Context::init(scenario.clone(), &src, &journal).expect("Context is created");
        #[allow(clippy::redundant_closure_call)]
        let result = AssertUnwindSafe($executing(elements, cx.clone(), journal.clone()))
            .catch_unwind()
            .await;
        let _ = cx.destroy().await;
        match result {
            Err(e) => {
                return exit(journal, &format!("{e:?}")).await;
            }
            Ok(Err(e)) => {
                return exit(journal, &e.to_string()).await;
            }
            _ => {}
        };
        let _ = journal.destroy().await;
    }};
}
