use crate::elements::{Element, InnersGetter, Values};
impl InnersGetter for Values {
    fn get_inners(&self) -> Vec<&Element> {
        self.elements.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{TokenGetter, Values},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?;
                assert!(entity.is_some(), "Line: {}", count + 1);
                let entity = entity.unwrap();
                assert_eq!(
                    trim_carets(reader.recent()),
                    trim_carets(&format!("{entity}")),
                    "Line: {}",
                    count + 1
                );
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn tokens() {
        let samples = include_str!("../../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?.unwrap();
                assert_eq!(
                    trim_carets(&entity.to_string()),
                    reader.get_fragment(&entity.token)?.lined,
                    "Line: {}",
                    count + 1
                );
                for el in entity.elements.iter() {
                    assert_eq!(
                        trim_carets(&el.to_string()),
                        trim_carets(&reader.get_fragment(&el.token())?.content),
                        "Line: {}",
                        count + 1
                    );
                }
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, _: &mut Sources| {
                let entity = Values::dissect(reader);
                assert!(entity.is_err());
                Ok::<usize, LinkedErr<E>>(1)
            });
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
    async fn reading() {
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
