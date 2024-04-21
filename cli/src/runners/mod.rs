use crate::{
    elements::Element,
    inf::{journal::Report, Configuration, Context, Journal, Scenario, Scope},
    reader::{Reader, Sources},
};
use futures_lite::FutureExt;
use std::{any::Any, panic::AssertUnwindSafe, process};

pub(crate) async fn exit<T>(journal: Journal, err: T)
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

        let journal = Journal::init(Configuration::logs());
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
        let journal = Journal::init(cfg);
        let mut src = Sources::new(&journal);
        let mut reader = src
            .reader()
            .unbound(&content)
            .expect("Unbound reader is created");
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
        use futures_lite::FutureExt;
        use std::{
            any::Any,
            panic::{self, AssertUnwindSafe},
        };
        use $crate::{inf::Scenario, runners::exit};

        let journal = Journal::init(Configuration::logs());
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting &Configuration as the first argument").await;
        };
        let content = $content as &dyn Any;
        let Some(content) = content.downcast_ref::<&str>().cloned() else {
            return exit(journal, "Expecting &content as the second argument").await;
        };
        let journal = Journal::init(cfg);
        let mut src = Sources::new(&journal);
        let mut reader = src
            .reader()
            .unbound(content)
            .expect("Unbound reader is created");

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
        let sc = Scope::init(Some(cx.scenario.filename.clone()));
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
macro_rules! read_file {
    ($cfg:expr, $scenario:expr, $executing:expr) => {{
        let journal = Journal::init(Configuration::logs());
        let cfg = $cfg as &dyn Any;
        let Some(cfg) = cfg.downcast_ref::<Configuration>().cloned() else {
            return exit(journal, "Expecting &Configuration as the first argument").await;
        };
        let scenario = $scenario as &dyn Any;
        let Some(scenario) = scenario.downcast_ref::<Scenario>().cloned() else {
            return exit(journal, "Expecting &Scenario as the second argument").await;
        };
        let journal = Journal::init(cfg);
        let mut src = Sources::new(&journal);
        let elements =
            match Reader::read_file(&scenario.filename, true, Some(&mut src), &journal).await {
                Ok(elements) => elements,
                Err(err) => {
                    return exit(journal, &err).await;
                }
            };
        let cx = Context::init(scenario.clone(), &src, &journal).expect("Context is created");
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

async fn test() {
    read_file!(
        &Configuration::logs(),
        &Scenario::dummy(),
        |elements: Vec<Element>, cx: Context, journal: Journal| async move {
            let _ = cx.atlas.clone();
            journal.destroy().await.map_err(|_| String::new())?;
            Ok::<(), String>(())
        }
    );
    read_string!(
        &Configuration::logs(),
        &String::from("test"),
        |reader: &mut Reader, src: &mut Sources| { Ok::<Vec<String>, String>(Vec::new()) }
    );
    process_string!(
        &Configuration::logs(),
        &String::from("test"),
        |reader: &mut Reader, src: &mut Sources| { Ok::<Vec<String>, String>(Vec::new()) },
        |output: Vec<String>, cx: Context, sc: Scope, journal: Journal| async move {
            Ok::<(), String>(())
        }
    )
}
