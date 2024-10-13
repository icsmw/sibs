use crate::inf::{journal::Report, Journal};
use std::process;

#[cfg(test)]
use crate::elements::ElementId;

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
pub async fn reading_ln_by_ln<S: AsRef<str>>(
    content: S,
    elements_ref: &[ElementId],
    exp_count: usize,
) {
    use crate::{
        elements::{Element, InnersGetter, TokenGetter},
        error::LinkedErr,
        inf::{operator::E, tests::trim_carets, Configuration},
        read_string,
        reader::{chars, Reader, Sources},
    };
    let cfg = Configuration::logs(false);
    let samples = content
        .as_ref()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let len = samples.len();
    let mut count = 0;
    let mut tokens = 0;
    for sample in samples.iter() {
        count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
            let Ok(result) = src.report_err_if(Element::include(reader, elements_ref)) else {
                panic!("Fail to read; line {}: sample:{:?}", count + 1, sample);
            };
            let Some(entity) = result else {
                panic!(
                    "Fail to get {:?}; line {}: sample:{:?}",
                    elements_ref,
                    count + 1,
                    sample
                );
            };
            let semicolon = reader.move_to().char(&[&chars::SEMICOLON]).is_some();
            // Compare generated and origin content
            assert_eq!(
                trim_carets(reader.recent()),
                trim_carets(&format!("{entity}{}", if semicolon { ";" } else { "" })),
                "Line: {}",
                count + 1
            );
            assert_eq!(
                trim_carets(sample),
                trim_carets(&format!("{entity}{}", if semicolon { ";" } else { "" })),
                "Line: {}",
                count + 1
            );
            // Checking self tokens
            assert_eq!(
                trim_carets(&format!("{entity}")),
                trim_carets(&reader.get_fragment(&entity.token())?.content)
            );
            tokens += 1;
            // Checking inners tokens
            for inner in entity.get_inners().iter() {
                assert_eq!(
                    trim_carets(&inner.to_string()),
                    trim_carets(&reader.get_fragment(&inner.token())?.lined),
                    "Line: {}",
                    count + 1
                );
                tokens += 1;
            }
            Ok::<usize, LinkedErr<E>>(1)
        });
    }
    assert_eq!(count, len);
    assert_eq!(count, exp_count);
    println!("[Errors Reading Test]: done for \"{elements_ref:?}\"; tested {count} element(s).");
}

#[cfg(test)]
#[macro_export]
macro_rules! test_reading_ln_by_ln {
    ($fn_name:ident, $content:expr, $element_ref:expr, $exp_count:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_ $fn_name >] () {
                $crate::runners::reading_ln_by_ln($content, $element_ref, $exp_count).await;
            }
        }
    };
}

#[cfg(test)]
pub async fn reading_el_by_el<S: AsRef<str>>(
    content: S,
    elements_ref: &[ElementId],
    exp_count: usize,
) {
    use crate::{
        elements::{Element, InnersGetter, TokenGetter},
        error::LinkedErr,
        inf::{operator::E, tests::trim_carets, Configuration},
        read_string,
        reader::{chars, Reader, Sources},
    };
    let content = content.as_ref().to_string();
    read_string!(
        &Configuration::logs(false),
        &content,
        |reader: &mut Reader, src: &mut Sources| {
            let mut count = 0;
            let mut tokens = 0;
            while let Some(entity) = src.report_err_if(Element::include(reader, elements_ref))? {
                let semicolon = reader.move_to().char(&[&chars::SEMICOLON]).is_some();
                // Compare generated and origin content
                assert_eq!(
                    trim_carets(reader.recent()),
                    trim_carets(&format!("{entity}{}", if semicolon { ";" } else { "" })),
                    "Line: {}",
                    count + 1
                );
                // Checking self tokens
                assert_eq!(
                    trim_carets(&format!("{entity}")),
                    trim_carets(&reader.get_fragment(&entity.token())?.content)
                );
                tokens += 1;
                // Checking inners tokens
                for inner in entity.get_inners().iter() {
                    assert_eq!(
                        trim_carets(&inner.to_string()),
                        trim_carets(&reader.get_fragment(&inner.token())?.lined),
                        "Line: {}",
                        count + 1
                    );
                    tokens += 1;
                }
                count += 1;
            }
            assert_eq!(count, exp_count);
            assert!(reader.rest().trim().is_empty());
            println!("[Reading Test]: done for \"{elements_ref:?}\"; tested {count} element(s); checked {tokens} token(s).");
            Ok::<(), LinkedErr<E>>(())
        }
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! test_reading_el_by_el {
    ($fn_name:ident, $content:expr, $element_ref:expr, $exp_count:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_ $fn_name >] () {
                $crate::runners::reading_el_by_el($content, $element_ref, $exp_count).await;
            }
        }
    };
}

#[cfg(test)]
pub async fn reading_with_errors_ln_by_ln<S: AsRef<str>>(
    content: S,
    elements_ref: &[ElementId],
    exp_count: usize,
) {
    use crate::{
        elements::Element,
        error::LinkedErr,
        inf::{operator::E, Configuration},
        read_string,
        reader::{chars, Reader, Sources},
    };
    let cfg = Configuration::logs(false);
    let samples = content
        .as_ref()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let len = samples.len();
    let mut count = 0;
    for sample in samples.iter() {
        count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
            let el = src.report_err_if(Element::include(reader, elements_ref));
            if let Ok(el) = el {
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                assert!(el.is_none(), "Line {}: element: {:?}", count + 1, el);
                assert!(
                    !reader.rest().trim().is_empty(),
                    "Line {}: element: {:?}",
                    count + 1,
                    el
                );
            } else {
                assert!(el.is_err(), "Line {}: func: {:?}", count + 1, el);
            }
            Ok::<usize, LinkedErr<E>>(1)
        });
    }
    assert_eq!(count, len);
    assert_eq!(count, exp_count);
    println!("[Errors Reading Test]: done for \"{elements_ref:?}\"; tested {count} element(s).");
}

#[cfg(test)]
#[macro_export]
macro_rules! test_reading_with_errors_ln_by_ln {
    ($fn_name:ident, $content:expr, $element_ref:expr, $exp_count:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_ $fn_name >] () {
                $crate::runners::reading_with_errors_ln_by_ln($content, $element_ref, $exp_count).await;
            }
        }
    };
}

#[cfg(test)]
pub async fn process_tasks_one_by_one<S: AsRef<str>, T>(
    content: S,
    expectation: T,
    exp_count: usize,
) where
    T: 'static + Clone + PartialEq + std::fmt::Debug,
{
    use crate::{
        elements::Element,
        error::LinkedErr,
        inf::{operator::E, Configuration, Context, Scope},
        process_string,
        reader::{chars, Reader, Sources},
    };
    let content = content.as_ref().to_string();
    process_string!(
        &Configuration::logs(false),
        &content,
        |reader: &mut Reader, src: &mut Sources| {
            let mut tasks: Vec<Element> = Vec::new();
            while let Some(task) =
                src.report_err_if(Element::include(reader, &[ElementId::Task]))?
            {
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                tasks.push(task);
            }
            Ok::<Vec<Element>, LinkedErr<E>>(tasks)
        },
        |tasks: Vec<Element>, _: Context, _: Scope, _: Journal| async move {
            assert_eq!(exp_count, tasks.len());
            for task in tasks.iter() {
                process_task(task.to_string(), expectation.clone()).await;
            }
            Ok::<(), LinkedErr<E>>(())
        }
    );
}

#[cfg(test)]
#[macro_export]
macro_rules! test_process_tasks_one_by_one {
    ($fn_name:ident, $content:expr, $expectation:expr, $exp_count:literal) => {
        paste::item! {
            #[tokio::test]
            async fn [< test_ $fn_name >] () {
                $crate::runners::process_tasks_one_by_one($content, $expectation, $exp_count).await;
            }
        }
    };
}

#[cfg(test)]
pub async fn process_task<S: AsRef<str>, T>(task: S, expectation: T)
where
    T: 'static + Clone + PartialEq + std::fmt::Debug,
{
    use crate::{
        elements::{Element, ElementId},
        error::LinkedErr,
        inf::{
            journal::Journal,
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, ExpectedValueType, Scope, Value,
        },
        process_string,
        reader::{Reader, Sources},
    };

    let comp = format!("#(app: ../) {}", task.as_ref());
    process_string!(
        &Configuration::logs(false),
        &comp,
        |reader: &mut Reader, src: &mut Sources| {
            let Some(component) =
                src.report_err_if(Element::include(reader, &[ElementId::Component]))?
            else {
                panic!("Fait to read component from:\n{comp}\n");
            };
            Ok::<Element, LinkedErr<E>>(component)
        },
        |component: Element, cx: Context, sc: Scope, _journal: Journal| async move {
            let result = component.linking(&component, &[], &None, &cx).await;
            if let Err(err) = result.as_ref() {
                cx.atlas
                    .report_err(err)
                    .await
                    .expect("Error report has been created");
            }
            assert!(result.is_ok());
            let result = component.verification(&component, &[], &None, &cx).await;
            if let Err(err) = result.as_ref() {
                cx.atlas
                    .report_err(err)
                    .await
                    .expect("Error report has been created");
            }
            assert!(result.is_ok());
            let result = component
                .execute(
                    ExecuteContext::unbound(cx.clone(), sc.clone())
                        .owner(Some(&component))
                        .args(&[Value::String("test".to_owned())]),
                )
                .await
                .or_else(|e| {
                    if e.e.is_aborted() {
                        Ok(Value::Empty(()))
                    } else {
                        Err(e)
                    }
                });
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
            Ok::<(), LinkedErr<E>>(())
        }
    );
}

#[cfg(test)]
pub async fn process_block<S: AsRef<str>, T>(block: S, expectation: T)
where
    T: 'static + Clone + PartialEq + std::fmt::Debug,
{
    process_task(format!("@test(){{{}}}", block.as_ref()), expectation).await;
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
