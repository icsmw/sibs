use crate::{
    elements::{Element, ElementRef, Gatekeeper, InnersGetter},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Gatekeeper {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.function.as_ref(), self.refs.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/gatekeeper.sibs"),
    ElementRef::Gatekeeper,
    3
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../tests/error/gatekeeper.sibs"),
    ElementRef::Gatekeeper,
    5
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
        reader::{Reader, Sources},
    };
    const CASES: &[&[(&[&str], Value)]] = &[
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::bool(true)),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::Empty(())),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::Empty(())),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::bool(true)),
        ],
    ];
    #[tokio::test]
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/gatekeeper.sibs"),
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
                for (n, component) in components.iter().enumerate() {
                    let case = CASES[n];
                    for (args, expected_result) in case.iter() {
                        let result = component
                            .execute(
                                ExecuteContext::unbound(cx.clone(), sc.clone())
                                    .args(
                                        &args
                                            .iter()
                                            .map(|s| Value::String(s.to_string()))
                                            .collect::<Vec<Value>>(),
                                    )
                                    .owner(Some(component))
                                    .components(&components),
                            )
                            .await
                            .expect("Component is executed");
                        assert_eq!(result, *expected_result);
                    }
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
