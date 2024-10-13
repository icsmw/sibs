use crate::{
    elements::{Element, ElementId, InnersGetter, VariableAssignation},
    test_reading_el_by_el, test_reading_with_errors_ln_by_ln,
};

impl InnersGetter for VariableAssignation {
    fn get_inners(&self) -> Vec<&Element> {
        vec![self.variable.as_ref(), self.assignation.as_ref()]
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../../tests/reading/variable_assignation.sibs"),
    &[ElementId::VariableAssignation],
    113
);

test_reading_with_errors_ln_by_ln!(
    errors,
    &include_str!("../../../tests/error/variable_assignation.sibs"),
    &[ElementId::VariableAssignation],
    3
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

    const VALUES: &[(&str, &str, bool)] = &[
        ("a", "a", false),
        ("b", "b", false),
        ("c", "abc", false),
        ("d", "ababc", false),
        ("e", "ababc", false),
        ("f", "\\{$a\\}\\{$b\\}\\{$c\\}", false),
        ("g", "\\{$a\\}\\{$b\\}\\{$c\\}", true),
    ];

    #[tokio::test]
    async fn processing() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/processing/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementId::Task]))?
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
                for (name, value, global) in VALUES.iter() {
                    assert_eq!(
                        if *global {
                            sc.get_global_var(name).await?
                        } else {
                            sc.get_var(name).await?
                        }
                        .unwrap()
                        .as_string()
                        .unwrap(),
                        value.to_string()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
