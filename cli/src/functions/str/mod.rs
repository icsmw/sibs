use crate::{
    elements::FuncArg,
    functions::{ExecutorFnDescription, ExecutorPinnedResult, TryAnyTo, E},
    inf::{Context, Scope, Store, Value, ValueRef},
};
use importer::import;

pub fn register(store: &mut Store<ExecutorFnDescription>) -> Result<(), E> {
    #[import(str)]
    fn repeat(target: String, count: usize) -> Result<String, E> {
        Ok(target.repeat(count))
    }
    #[import(str)]
    fn to_ascii_lowercase(target: String) -> Result<String, E> {
        Ok(target.to_ascii_lowercase())
    }
    #[import(str)]
    fn to_ascii_uppercase(target: String) -> Result<String, E> {
        Ok(target.to_ascii_uppercase())
    }
    #[import(str)]
    fn to_lowercase(target: String) -> Result<String, E> {
        Ok(target.to_lowercase())
    }
    #[import(str)]
    fn to_uppercase(target: String) -> Result<String, E> {
        Ok(target.to_uppercase())
    }
    #[import(str)]
    fn replace(target: String, old: String, new: String) -> Result<String, E> {
        Ok(target.replace(old.as_str(), &new))
    }
    #[import(str)]
    fn sub(target: String, from: usize, count: usize) -> Result<String, E> {
        let len = target.chars().count();
        if from >= len {
            return Ok(String::new());
        }
        let available_count = len - from;
        Ok(target
            .chars()
            .skip(from)
            .take(count.min(available_count))
            .collect())
    }
    #[import(str)]
    fn split_off(mut target: String, at: usize) -> Result<String, E> {
        Ok(target.split_off(at))
    }
    #[import(str)]
    fn trim(target: String) -> Result<String, E> {
        Ok(target.trim().to_string())
    }
    #[import(str)]
    fn trim_end(target: String) -> Result<String, E> {
        Ok(target.trim_end().to_string())
    }
    #[import(str)]
    fn trim_start(target: String) -> Result<String, E> {
        Ok(target.trim_start().to_string())
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use journal::Journal;
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            journal,
            operator::{Execute, E},
            Configuration, Context, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    const TESTS: &[&str] = &[
        r#"
            if "R".repeat(5) == "RRRRR" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "R".to_ascii_lowercase() == "r" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "r".to_ascii_uppercase() == "R" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "R".to_lowercase() == "r" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "r".to_uppercase() == "R" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            $a = "Hello World!";
            $b = $a.sub(0, 5);
            $c = $a.str::sub(0, 5).str::sub(0, 2);
            if $b == "Hello" && $c == "He" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "Hello, World!".split_off(7) == "World!" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "   word   ".trim() == "word" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "   word   ".trim_end() == "   word" {
                true;
            } else {
                false;
            };
        "#,
        r#"
            if "   word   ".trim_start() == "word   " {
                true;
            } else {
                false;
            };
        "#,
    ];

    #[tokio::test]
    async fn reading() {
        for test in TESTS.iter() {
            process_string!(
                &Configuration::logs(false),
                &format!("@test(){{{test}}}"),
                |reader: &mut Reader, src: &mut Sources| {
                    let mut tasks: Vec<Element> = Vec::new();
                    while let Some(task) =
                        src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
                    {
                        let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                        tasks.push(task);
                    }
                    Ok::<Vec<Element>, LinkedErr<E>>(tasks)
                },
                |tasks: Vec<Element>, cx: Context, sc: Scope, _journal: Journal| async move {
                    assert!(!tasks.is_empty());
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
                        assert!(result
                            .expect("run of task is success")
                            .as_bool()
                            .expect("test returns bool value"));
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        }
    }
}
