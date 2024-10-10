use crate::elements::{Component, Element, InnersGetter};

#[cfg(test)]
impl InnersGetter for Component {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Component, Element, ElementRef, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let components = include_str!("../../tests/reading/component.sibs")
            .split('\n')
            .collect::<Vec<&str>>();
        let tasks = include_str!("../../tests/reading/tasks.sibs");
        read_string!(
            &Configuration::logs(false),
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Component::dissect(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string()),
                    );
                    count += 1;
                }
                assert_eq!(count, components.len());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        let components = include_str!("../../tests/reading/component.sibs")
            .split('\n')
            .collect::<Vec<&str>>();
        let tasks = include_str!("../../tests/reading/tasks.sibs");
        read_string!(
            &Configuration::logs(false),
            &components
                .iter()
                .map(|c| format!("{c}\n{tasks}"))
                .collect::<Vec<String>>()
                .join("\n"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(el) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Component]))?
                {
                    assert!(matches!(el, Element::Component(..)));
                    assert_eq!(
                        trim_carets(&el.to_string()),
                        trim_carets(&reader.get_fragment(&el.token())?.lined)
                    );
                    if let Element::Component(el, _) = el {
                        assert_eq!(
                            trim_carets(&el.name.to_string()),
                            trim_carets(&reader.get_fragment(&el.name.token)?.lined)
                        );
                        for el in el.elements.iter() {
                            if let Element::Task(el, _) = el {
                                assert_eq!(
                                    trim_carets(&format!("{el}",)),
                                    trim_carets(&reader.get_fragment(&el.token())?.lined)
                                );
                            } else {
                                assert_eq!(
                                    trim_carets(&format!("{el}",)),
                                    trim_carets(&reader.get_fragment(&el.token())?.lined)
                                );
                            }
                        }
                    }
                    count += 1;
                }
                assert_eq!(count, components.len());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/component.sibs");
        let samples = samples
            .split('\n')
            .map(|v| format!("{v} task {{\nenv::is_os();\n}};"))
            .collect::<Vec<String>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let res = Component::dissect(reader);
                    println!("{res:?}");
                    assert!(res.is_err());
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
        reader::{Reader, Sources},
    };

    const VALUES: &[&[&str]] = &[
        &["test", "a"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
        &["test", "a", "b", "c"],
    ];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/component.sibs"),
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
                for (i, component) in components.iter().enumerate() {
                    let result = component
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone())
                                .owner(Some(component))
                                .components(&components)
                                .args(
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
}
