use crate::{
    elements::{Component, Element, ElementId, InnersGetter},
    runners::{reading_el_by_el, reading_with_errors_ln_by_ln},
};

impl InnersGetter for Component {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[tokio::test]
async fn reading() {
    let tasks = include_str!("../../tests/reading/tasks.sibs");
    let content = include_str!("../../tests/reading/component.sibs")
        .split('\n')
        .map(|c| format!("{c}\n{tasks}"))
        .collect::<Vec<String>>()
        .join("\n");
    reading_el_by_el(&content, &[ElementId::Component], 6).await;
}

#[tokio::test]
async fn errors() {
    let content = include_str!("../../tests/error/component.sibs")
        .split('\n')
        .map(|c| format!("{c} task {{env::is_os();}};"))
        .collect::<Vec<String>>()
        .join("\n");
    reading_with_errors_ln_by_ln(&content, &[ElementId::Component], 16).await;
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementId},
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
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/component.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::read(reader, &[ElementId::Component]))?
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
