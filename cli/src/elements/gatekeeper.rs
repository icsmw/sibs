use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element, Reference},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter,
        TryExecute, Value, ValueRef, VerificationResult,
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
        owner: Option<&'a Element>,
        components: &'a [Element],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
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
                .execute(
                    owner,
                    components,
                    &[],
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::NoValueFromGatekeeper)?
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
        let close = reader.open_token(ElTarget::Gatekeeper);
        let function = if let Some(el) = Element::include(reader, &[ElTarget::Function])? {
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

impl Dissect<Gatekeeper, Gatekeeper> for Gatekeeper {}

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

impl TokenGetter for Gatekeeper {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Gatekeeper {
    fn varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }
    fn linking<'a>(
        &'a self,
        _variables: &'a mut GlobalVariablesMap,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { Ok(()) })
    }

    fn expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl TryExecute for Gatekeeper {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let condition = *self
                .function
                .execute(
                    owner,
                    components,
                    args,
                    prev,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            Ok(Some(Value::bool(condition)))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Gatekeeper,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
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
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{Component, ElTarget, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope, Value,
        },
        process_string,
        reader::{chars, Dissect, Reader, Sources},
    };
    const CASES: &[&[(&[&str], Option<bool>)]] = &[
        &[(&["test", "a"], None), (&["test", "b"], Some(true))],
        &[(&["test", "a"], None), (&["test", "b"], None)],
        &[(&["test", "a"], None), (&["test", "b"], None)],
        &[(&["test", "a"], None), (&["test", "b"], Some(true))],
    ];
    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/gatekeeper.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Component]))?
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
                                Some(component),
                                &components,
                                &args
                                    .iter()
                                    .map(|s| Value::String(s.to_string()))
                                    .collect::<Vec<Value>>(),
                                &None,
                                cx.clone(),
                                sc.clone(),
                                CancellationToken::new(),
                            )
                            .await
                            .expect("Component is executed");
                        assert_eq!(result.is_some(), expected_result.is_some());
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
        elements::{Component, ElTarget, Element, Gatekeeper},
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
