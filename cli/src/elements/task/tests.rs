use crate::{
    elements::{Element, ElementRef, InnersGetter, Task},
    test_reading_el_by_el,
};

impl InnersGetter for Task {
    fn get_inners(&self) -> Vec<&Element> {
        [
            self.declarations.iter().collect(),
            self.dependencies.iter().collect(),
            vec![self.block.as_ref()],
        ]
        .concat()
    }
}

// test_reading_el_by_el!(
//     reading,
//     &include_str!("../../tests/reading/tasks.sibs"),
//     ElementRef::Task,
//     11
// );
// reader recent in use

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Element, ElementRef, Task, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/tasks.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    assert!(matches!(el, Element::Task(..)));
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(trim_carets(reader.recent()), trim_carets(&format!("{el};")));
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 11);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn deps() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/deps.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    assert!(matches!(el, Element::Task(..)));
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(trim_carets(reader.recent()), trim_carets(&format!("{el};")));
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 1);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/tasks.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert!(matches!(el, Element::Task(..)));
                    if let Element::Task(el, _) = el {
                        assert_eq!(
                            trim_carets(&format!("{el}")),
                            trim_carets(&reader.get_fragment(&el.token)?.lined)
                        );
                        assert_eq!(
                            trim_carets(&el.name.value),
                            trim_carets(&reader.get_fragment(&el.name.token)?.lined)
                        );
                        assert_eq!(
                            trim_carets(&el.block.to_string()),
                            trim_carets(&reader.get_fragment(&el.block.token())?.lined)
                        );
                        for declaration in el.declarations.iter() {
                            assert_eq!(
                                trim_carets(&declaration.to_string()),
                                trim_carets(&reader.get_fragment(&declaration.token())?.lined)
                            );
                        }
                        for dependency in el.dependencies.iter() {
                            assert_eq!(
                                trim_carets(&dependency.to_string()),
                                trim_carets(&reader.get_fragment(&dependency.token())?.lined)
                            );
                        }
                    }
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                assert_eq!(count, 11);
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/tasks.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Task::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope, Value,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    const VALUES: &[&[&str]] = &[&["a"], &["a", "b"], &["a"], &["a", "b"], &["a", "b", "c"]];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/tasks.sibs"),
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
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for (i, task) in tasks.iter().enumerate() {
                    let result = task
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone()).args(
                                &VALUES[i]
                                    .iter()
                                    .map(|s| Value::String(s.to_string()))
                                    .collect::<Vec<Value>>(),
                            ),
                        )
                        .await?;
                    assert_eq!(
                        result.as_string().expect("Task returns string value"),
                        "true".to_owned()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn deps() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/deps.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |components: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for component in components.iter() {
                    if !component.as_component()?.name.value.ends_with("_run") {
                        continue;
                    }
                    let result = component
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone())
                                .owner(Some(component))
                                .components(&components)
                                .args(&[
                                    Value::String("test".to_owned()),
                                    Value::String("a".to_owned()),
                                    Value::String("b".to_owned()),
                                ]),
                        )
                        .await?;
                    assert_eq!(
                        result.as_string().expect("Task returns string value"),
                        "true".to_owned()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
