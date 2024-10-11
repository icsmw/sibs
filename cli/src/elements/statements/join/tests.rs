use crate::elements::{Element, InnersGetter, Join};
impl InnersGetter for Join {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.elements.as_ref()]
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Join, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.elements.to_string()),
                        trim_carets(&reader.get_fragment(&entity.elements.token())?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../../tests/error/join.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Join::dissect(reader).is_err());
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
        elements::Element,
        error::LinkedErr,
        inf::{
            journal::{Configuration, Journal},
            Context, Execute, ExecuteContext, Scope, Value,
        },
        process_file,
        reader::{error::E, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/processing/join.sibs");
        process_file!(
            &Configuration::logs(false),
            &target,
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                assert_eq!(elements.len(), 1);
                let Some(el) = elements.first() else {
                    panic!("Component isn't found");
                };
                let results = el
                    .execute(
                        ExecuteContext::unbound(cx.clone(), sc.clone())
                            .args(&[Value::String(String::from("test_a"))]),
                    )
                    .await
                    .expect("run is successfull");
                assert!(results.get::<Vec<Value>>().is_some());
                assert_eq!(
                    results
                        .get::<Vec<Value>>()
                        .expect("join returns Vec<Value>")
                        .len(),
                    4
                );
                let results = el
                    .execute(
                        ExecuteContext::unbound(cx.clone(), sc.clone())
                            .owner(Some(el))
                            .args(&[Value::String(String::from("test_b"))]),
                    )
                    .await
                    .expect("run is successfull");
                assert!(results.get::<Vec<Value>>().is_some());
                assert_eq!(
                    results
                        .get::<Vec<Value>>()
                        .expect("join returns Vec<Value>")
                        .len(),
                    2
                );
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn errors() {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/processing/join.sibs");
        process_file!(
            &Configuration::logs(false),
            &target,
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                assert_eq!(elements.len(), 1);
                let Some(el) = elements.first() else {
                    panic!("Component isn't found");
                };
                for task in &["test_c", "test_d", "test_e", "test_f", "test_g", "test_j"] {
                    assert!(el
                        .execute(
                            ExecuteContext::unbound(cx.clone(), sc.clone())
                                .owner(Some(el))
                                .args(&[Value::String(task.to_string())])
                        )
                        .await
                        .expect("task is done successfuly")
                        .as_bool()
                        .expect("return value is bool"));
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
