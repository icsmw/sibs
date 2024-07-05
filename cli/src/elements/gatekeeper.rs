use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
    },
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Gatekeeper {
    pub function: Box<Element>,
    pub refs: Box<Element>,
    pub token: usize,
}

impl Reading<Gatekeeper> for Gatekeeper {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::REF_TO) {
            return Ok(None);
        }
        let restore = reader.pin();
        let close = reader.open_token();
        let function = if let Some(el) = Element::include(reader, &[ElTarget::Function])? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        if !reader.rest().trim().starts_with(words::REF_TO) {
            restore(reader);
            return Ok(None);
        }
        if reader.move_to().expression(&[words::REF_TO]).is_none() {
            return Err(E::NoReferenceForGatekeeper.by_reader(reader));
        }
        let Some(refs) = Element::include(reader, &[ElTarget::Values, ElTarget::Reference])? else {
            return Err(E::NoReferenceForGatekeeper.by_reader(reader));
        };
        match &refs {
            Element::Reference(..) => {}
            Element::Values(values, _) => {
                if values
                    .elements
                    .iter()
                    .any(|el| !matches!(el, Element::Reference(..)))
                {
                    return Err(E::GatekeeperShouldRefToTask.by_reader(reader));
                }
            }
            _ => {
                return Err(E::GatekeeperShouldRefToTask.by_reader(reader));
            }
        }
        Ok(Some(Gatekeeper {
            token: close(reader),
            function,
            refs: Box::new(refs),
        }))
    }
}

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.function, self.refs)
    }
}

impl Formation for Gatekeeper {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Gatekeeper));
        format!(
            "{}{} -> {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.function.format(&mut inner),
            self.refs.format(&mut inner),
        )
    }
}

impl Operator for Gatekeeper {
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
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let condition = *self
                .function
                .execute(
                    owner,
                    components,
                    args,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            Ok(Some(AnyValue::bool(condition)))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Gatekeeper,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Gatekeeper::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Gatekeeper::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.lined,
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.function.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.function.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.refs.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.refs.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/gatekeeper.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let opt = Gatekeeper::read(reader);
                    assert!(opt.is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Reading, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/optional.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Task>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    let result = task
                        .execute(
                            None,
                            &[],
                            &[],
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new(),
                        )
                        .await?
                        .expect("Task returns some value");
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

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Component, ElTarget, Element, Gatekeeper},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Reader, Reading, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Gatekeeper {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElTarget::Function], deep)),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::Reference]
                    } else {
                        // TODO: should be added ElTarget::Values with references only
                        vec![ElTarget::Reference]
                    },
                    deep,
                )),
            )
                .prop_map(|(function, action)| Gatekeeper {
                    function: Box::new(function),
                    refs: Box::new(action),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(gatekeeper: Gatekeeper) {
        get_rt().block_on(async {
            let origin = format!("#(test: ./){gatekeeper};\ntest [\n@print(\"hello world\");\n];");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(component) = src.report_err_if(Component::read(reader))? {
                        assert_eq!(format!("{component}"), origin);
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
            args in any_with::<Gatekeeper>(0)
        ) {
            reading(args.clone());
        }
    }
}
