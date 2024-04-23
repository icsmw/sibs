use tokio::{spawn, task::JoinHandle};

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Formation, FormationCursor, Operator, OperatorPinnedResult, OperatorResult, Scope,
    },
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Join {
    pub elements: Box<Element>,
    pub token: usize,
}

impl Reading<Join> for Join {
    fn read(reader: &mut Reader) -> Result<Option<Join>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::JOIN]).is_some() {
            let Some(Element::Values(elements, md)) =
                Element::include(reader, &[ElTarget::Values])?
            else {
                return Err(E::NoJOINStatementBody.by_reader(reader));
            };
            if elements.elements.is_empty() {
                Err(E::NoJOINStatementBody.by_reader(reader))?;
            }
            for el in elements.elements.iter() {
                if !matches!(
                    el,
                    Element::Reference(..) | Element::Function(..) | Element::Command(..)
                ) {
                    Err(E::NotReferenceInJOIN.linked(&el.token()))?;
                }
            }
            Ok(Some(Join {
                elements: Box::new(Element::Values(elements, md)),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JOIN {}", self.elements)
    }
}

impl Formation for Join {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Join));
        format!(
            "{}JOIN {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.elements.format(&mut inner)
        )
    }
}

impl Operator for Join {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: Context,
        sc: Scope,
    ) -> OperatorPinnedResult {
        fn clone(
            owner: Option<&Component>,
            components: &[Component],
            args: &[String],
            cx: &Context,
            sc: &Scope,
        ) -> (
            Option<Component>,
            Vec<Component>,
            Vec<String>,
            Context,
            Scope,
        ) {
            (
                owner.cloned().clone(),
                components.to_vec(),
                args.to_vec(),
                cx.clone(),
                sc.clone(),
            )
        }
        Box::pin(async move {
            let Element::Values(values, _) = self.elements.as_ref() else {
                return Ok(None);
            };
            let tasks = values
                .elements
                .iter()
                .cloned()
                .map(|el| {
                    let params = clone(owner, components, args, &cx, &sc);
                    spawn(async move {
                        el.execute(params.0.as_ref(), &params.1, &params.2, params.3, params.4)
                            .await
                    })
                })
                .collect::<Vec<JoinHandle<OperatorResult>>>();
            for task in tasks {
                task.await;
            }
            // TODO:
            // - collect results
            // - test with references to tasks
            // - abort on error of some task
            // - collect logs of each task
            Ok(None)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Join,
        error::LinkedErr,
        inf::{tests::*, Configuration, Operator},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(),
            &include_str!("../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::read(reader))? {
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
            &Configuration::logs(),
            &include_str!("../../tests/reading/join.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Join::read(reader))? {
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
        let samples = include_str!("../../tests/error/join.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Join::read(reader).is_err());
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
            Context, Operator, Scope,
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
            &Configuration::logs(),
            &target,
            |elements: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                assert_eq!(elements.len(), 1);
                let Some(Element::Component(el, _md)) = elements.first() else {
                    panic!("Component isn't found");
                };
                assert!(el
                    .execute(None, &[], &[String::from("test")], cx, sc)
                    .await
                    .is_ok());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Join, Metadata, Task, Values},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Reader, Reading, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Join {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::Reference]
                    } else {
                        vec![ElTarget::Reference, ElTarget::Function, ElTarget::Command]
                    },
                    deep,
                )),
                1..=10,
            )
            .prop_map(|elements| Values { elements, token: 0 })
            .prop_map(|elements| Join {
                elements: Box::new(Element::Values(elements, Metadata::empty())),
                token: 0,
            })
            .boxed()
        }
    }

    fn reading(join: Join) {
        get_rt().block_on(async {
            let origin = format!("test [\n{join};\n];");
            read_string!(
                &Configuration::logs(),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(task) = src.report_err_if(Task::read(reader))? {
                        assert_eq!(format!("{task};"), origin);
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_task(
            args in any_with::<Join>(0)
        ) {
            reading(args.clone());
        }
    }
}
