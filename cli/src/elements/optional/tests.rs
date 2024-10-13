use crate::{
    elements::{Element, ElementRef, InnersGetter, Optional},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for Optional {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.condition.as_ref(), self.action.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/optional.sibs"),
    ElementRef::Optional,
    106
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../tests/error/optional.sibs"),
    ElementRef::Optional,
    7
);

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/optional.sibs"),
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
                for task in tasks.iter() {
                    let result = task
                        .execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
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
