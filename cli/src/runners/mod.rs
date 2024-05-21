use crate::inf::{journal::Report, Journal};
use std::process;

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
        let sc = Scope::init(Some(cx.scenario.path.clone()), &journal);
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
        let sc = Scope::init(Some(cx.scenario.path.clone()), &journal);
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
