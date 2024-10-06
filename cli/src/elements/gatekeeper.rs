use crate::{
    elements::{Element, ElementRef, Reference, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult, Formation,
        FormationCursor, LinkingResult, PrevValueExpectation, Processing, TryExecute,
        TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Gatekeeper {
    pub function: Box<Element>,
    pub refs: Box<Element>,
    pub token: usize,
}

impl Gatekeeper {
    pub fn get_refs(&self) -> Vec<&Reference> {
        let mut refs = Vec::new();
        let Element::Values(values, _) = self.refs.as_ref() else {
            unreachable!("References can be stored only in Values of Gatekeeper")
        };
        for el in values.elements.iter() {
            let Element::Reference(reference, _) = el else {
                unreachable!("Only references can be stored in Gatekeeper")
            };
            refs.push(reference);
        }
        refs
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn skippable<'a>(
        gatekeepers: Vec<&Element>,
        task_ref: &Reference,
        cx: ExecuteContext<'a>,
    ) -> Result<bool, LinkedErr<operator::E>> {
        if gatekeepers.is_empty() {
            return Ok(false);
        }
        for el in gatekeepers.iter() {
            let Element::Gatekeeper(gatekeeper, _) = el else {
                continue;
            };
            let refs = gatekeeper.get_refs();
            if !refs.is_empty() && !refs.iter().any(|reference| reference == &task_ref) {
                return Ok(false);
            }
            // On "true" - task should be done; on "false" - can be skipped.
            if el
                .execute(cx.clone().args(&[]))
                .await?
                .as_bool()
                .ok_or(operator::E::NoBoolValueFromGatekeeper)?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
impl TryDissect<Gatekeeper> for Gatekeeper {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::REF_TO) {
            return Ok(None);
        }
        let close = reader.open_token(ElementRef::Gatekeeper);
        let function = if let Some(el) = Element::include(reader, &[ElementRef::Function])? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        if !reader.rest().trim().starts_with(words::REF_TO) {
            return Ok(None);
        }
        if reader.move_to().expression(&[words::REF_TO]).is_none() {
            return Err(E::NoReferenceForGatekeeper.by_reader(reader));
        }
        let Some(refs) =
            Element::include(reader, &[ElementRef::Values, ElementRef::Reference])?
        else {
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

impl Dissect<Gatekeeper, Gatekeeper> for Gatekeeper {}

impl fmt::Display for Gatekeeper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.function, self.refs)
    }
}

impl Formation for Gatekeeper {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Gatekeeper));
        format!(
            "{}{} -> {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.function.format(&mut inner),
            self.refs.format(&mut inner),
        )
    }
}

impl TokenGetter for Gatekeeper {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Gatekeeper {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
    }

    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl Processing for Gatekeeper {}

impl TryExecute for Gatekeeper {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let condition = *self
                .function
                .execute(cx.clone())
                .await?
                .get::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            Ok(Value::bool(condition))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Gatekeeper,TokenGetter},
        error::LinkedErr,
        inf::{ tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Gatekeeper::dissect(reader))? {
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
                while let Some(entity) = src.report_err_if(Gatekeeper::dissect(reader))? {
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
                    let opt = Gatekeeper::dissect(reader);
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
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope, Value,
        },
        process_string,
        reader::{Reader, Sources},
    };
    const CASES: &[&[(&[&str], Value)]] = &[
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::bool(true)),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::Empty(())),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::Empty(())),
        ],
        &[
            (&["test", "a"], Value::Empty(())),
            (&["test", "b"], Value::bool(true)),
        ],
    ];
    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElementRef::Component]))?
                {
                    components.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(components)
            },
            |components: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for (n, component) in components.iter().enumerate() {
                    let case = CASES[n];
                    for (args, expected_result) in case.iter() {
                        let result = component
                            .execute(
                                ExecuteContext::unbound(cx.clone(), sc.clone())
                                    .args(
                                        &args
                                            .iter()
                                            .map(|s| Value::String(s.to_string()))
                                            .collect::<Vec<Value>>(),
                                    )
                                    .owner(Some(component))
                                    .components(&components),
                            )
                            .await
                            .expect("Component is executed");
                        assert_eq!(result, *expected_result);
                    }
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Component, Element, ElementRef, Gatekeeper},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Gatekeeper {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElementRef::Function], deep)),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElementRef::Reference]
                    } else {
                        // TODO: should be added ElementRef::Values with references only
                        vec![ElementRef::Reference]
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
            let origin =
                format!("#(test: ./){gatekeeper};\n@test {{\nprint(\"hello world\");\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(component) = src.report_err_if(Component::dissect(reader))? {
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
