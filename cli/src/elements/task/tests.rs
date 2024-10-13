use crate::{
    elements::{Element, ElementRef, InnersGetter, Task},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
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

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/tasks.sibs"),
    &[ElementRef::Task],
    12
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../tests/error/tasks.sibs"),
    &[ElementRef::Task],
    9
);

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
    async fn processing() {
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
