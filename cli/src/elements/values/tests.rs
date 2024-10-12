use crate::{
    elements::{Element, ElementRef, InnersGetter, Values},
    test_reading_el_by_el, test_reading_with_errors_line_by_line,
};

impl InnersGetter for Values {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

test_reading_el_by_el!(
    reading,
    &include_str!("../../tests/reading/values.sibs"),
    ElementRef::Values,
    62
);

test_reading_with_errors_line_by_line!(
    errors,
    &include_str!("../../tests/error/values.sibs"),
    ElementRef::Values,
    7
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
        process_string, read_string,
        reader::{chars, Reader, Sources},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a,b"),
        ("a2", "a,b,c"),
        ("a3", "a,b,c"),
        ("a4", "aa,bb,cc"),
        ("a5", "a:a,b:b"),
    ];
    const NESTED_VALUES: &[(&str, &str)] = &[("a6", "c:a,d:b,d:c")];

    #[tokio::test]
    async fn processing() {
        let components: Vec<Element> = read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/values_components.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            }
        );
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/values.sibs"),
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
                    task.execute(
                        ExecuteContext::unbound(cx.clone(), sc.clone())
                            .owner(components.first())
                            .components(&components),
                    )
                    .await?;
                }
                for (name, value) in VALUES.iter() {
                    assert_eq!(
                        sc.get_var(name)
                            .await?
                            .unwrap()
                            .as_strings()
                            .unwrap()
                            .join(","),
                        value.to_string()
                    );
                }
                for (name, value) in NESTED_VALUES.iter() {
                    let stored = sc.get_var(name).await?.unwrap();
                    let values = stored.get::<Vec<Value>>().unwrap();
                    let mut output: Vec<String> = Vec::new();
                    for value in values.iter() {
                        output = [output, value.as_strings().unwrap()].concat();
                    }
                    assert_eq!(output.join(","), value.to_string());
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}
