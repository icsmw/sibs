use crate::elements::{Element, Gatekeeper, InnersGetter};

impl InnersGetter for Gatekeeper {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.function.as_ref(), self.refs.as_ref()]
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Gatekeeper, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Gatekeeper::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Gatekeeper::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.lined,
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.function.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.function.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.refs.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.refs.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/gatekeeper.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let opt = Gatekeeper::dissect(reader);
                    assert!(opt.is_err());
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
    async fn reading() {
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
