use crate::{
    elements::{Element, ElementRef, InnersGetter, Join},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Join {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.elements.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/join.sibs"),
    ElementRef::Join,
    2
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/join.sibs"),
    ElementRef::Join,
    2
);

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
    async fn processing() {
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
