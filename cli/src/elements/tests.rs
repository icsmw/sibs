use crate::{
    elements::{Element, ElementId},
    error::LinkedErr,
    inf::{
        operator::{Execute, E},
        Configuration, Context, ExecuteContext, Journal, Scope,
    },
    process_string,
    reader::{chars, Reader, Sources},
    test_reading_el_by_el,
};

test_reading_el_by_el!(
    reading,
    &include_str!("../tests/reading/ppm.sibs"),
    &[ElementId::Function, ElementId::VariableName],
    94
);

#[tokio::test]
async fn processing() {
    process_string!(
        &Configuration::logs(false),
        &include_str!("../tests/processing/tolerance.sibs"),
        |reader: &mut Reader, src: &mut Sources| {
            let mut elements: Vec<Element> = Vec::new();
            while let Some(el) = src.report_err_if(Element::read(reader, &[ElementId::Task]))? {
                let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                elements.push(el);
            }
            Ok::<Vec<Element>, LinkedErr<E>>(elements)
        },
        |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
            for el in elements.iter() {
                el.execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                    .await?;
            }
            Ok::<(), LinkedErr<E>>(())
        }
    );
}
