use crate::{
    elements::{Element, ElementRef, If, InnersGetter},
    test_reading_el_by_el, test_reading_with_errors_line_by_line,
};

impl InnersGetter for If {
    fn get_inners(&self) -> Vec<&Element> {
        self.threads
            .iter()
            .flat_map(|thr| thr.get_inners())
            .collect()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/if.sibs"),
    ElementRef::If,
    90
);

test_reading_with_errors_line_by_line!(
    errors,
    &include_str!("../../../tests/error/if.sibs"),
    ElementRef::If,
    15
);

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, ExecuteContext, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn processing() {
        let tasks_count = include_str!("../../../tests/processing/if.sibs")
            .match_indices(chars::AT)
            .count();
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/processing/if.sibs"),
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
                        .execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                        .await?;
                    assert_eq!(
                        result.as_string().expect("if returns string value"),
                        "true".to_owned(),
                        "Line: {}",
                        i + 1
                    );
                }
                assert_eq!(tasks_count, tasks.len());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
