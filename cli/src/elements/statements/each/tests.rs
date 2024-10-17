use crate::{
    elements::{Each, Element, ElementId, InnersGetter},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};
impl InnersGetter for Each {
    fn get_inners(&self) -> Vec<&Element> {
        vec![
            self.block.as_ref(),
            self.input.as_ref(),
            self.variable.as_ref(),
        ]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/each.sibs"),
    &[ElementId::Each],
    7
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/each.sibs"),
    &[ElementId::Each],
    10
);

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Element, ElementId},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[tokio::test]
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/processing/each.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::read(reader, &[ElementId::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    task.execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                        .await?;
                }
                for (name, value) in VALUES.iter() {
                    assert_eq!(
                        sc.get_var(name).await?.unwrap().as_string().unwrap(),
                        value.to_string()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
