use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, VerificationResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct Each {
    pub variable: Box<Element>,
    pub input: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}

impl TryDissect<Each> for Each {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Each>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Each);
        if reader.move_to().word(&[words::EACH]).is_some() {
            let (variable, input) = if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let variable = if let Some(variable) =
                    Element::include(&mut inner, &[ElementRef::VariableName])?
                {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon.linked(&inner.token()?.id));
                    }
                    variable
                } else {
                    return Err(E::NoLoopVariable.linked(&inner.token()?.id));
                };
                let input = if let Some(el) = Element::include(
                    &mut inner,
                    &[ElementRef::Function, ElementRef::VariableName],
                )? {
                    Box::new(el)
                } else {
                    Err(E::NoLoopInput.by_reader(&inner))?
                };
                (variable, input)
            } else {
                return Err(E::NoLoopInitialization.linked(&reader.token()?.id));
            };
            let Some(mut block) = Element::include(reader, &[ElementRef::Block])? else {
                Err(E::NoGroup.by_reader(reader))?
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementRef::Each);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Each {
                input,
                variable: Box::new(variable),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Each, Each> for Each {}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "each({}; {}) {}", self.variable, self.input, self.block)
    }
}

impl Formation for Each {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}each({}; {}) {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.variable,
            self.input,
            self.block.format(cursor)
        )
    }
}

impl TokenGetter for Each {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Each {
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
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            self.input.linking(owner, components, prev, cx).await?;
            self.block.linking(owner, components, prev, cx).await?;
            self.variable.linking(owner, components, prev, cx).await?;
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { self.block.expected(owner, components, prev, cx).await })
    }
}

impl Processing for Each {}

impl TryExecute for Each {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let inputs = self
                .input
                .execute(cx.clone())
                .await?
                .as_strings()
                .ok_or(operator::E::FailConvertInputIntoStringsForEach)?;
            let mut output: Value = Value::empty();
            let blk_token = if let Element::Block(el, _) = self.block.as_ref() {
                el.get_breaker()?
            } else {
                return Err(operator::E::BlockElementExpected.linked(&self.block.token()));
            };
            let (loop_uuid, loop_token) = cx.sc.open_loop(blk_token).await?;
            let Element::VariableName(variable, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self.variable.as_ref()));
            };
            for iteration in inputs.iter() {
                if loop_token.is_cancelled() {
                    break;
                }
                cx.sc
                    .set_var(&variable.name, Value::String(iteration.to_string()))
                    .await?;
                output = self.block.execute(cx.clone()).await?;
            }
            cx.sc.close_loop(loop_uuid).await?;
            Ok(output)
        })
    }
}

#[cfg(test)]
use crate::elements::InnersGetter;

#[cfg(test)]
impl InnersGetter for Each {
    fn get_inners(&self) -> Vec<&Element> {
        vec![
            self.block.as_ref(),
            self.input.as_ref(),
            self.variable.as_ref(),
        ]
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Each, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/each.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Each::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 7);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/each.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Each::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token())?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.variable.to_string()),
                        trim_carets(&reader.get_fragment(&entity.variable.token())?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.input.to_string()),
                        trim_carets(&reader.get_fragment(&entity.input.token())?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 7);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/each.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(Each::dissect(reader).is_err());
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
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/each.sibs"),
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

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{task::Task, Each, Element, ElementRef},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Each {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElementRef::Block], deep)),
                Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
                Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
            )
                .prop_map(|(block, variable, input)| Each {
                    block: Box::new(block),
                    variable: Box::new(variable),
                    input: Box::new(input),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(each: Each) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{each};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    let task = src
                        .report_err_if(Task::dissect(reader))?
                        .expect("Task read");
                    assert_eq!(format!("{task};"), origin);
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
            args in any_with::<Each>(0)
        ) {
            reading(args.clone());
        }
    }
}
