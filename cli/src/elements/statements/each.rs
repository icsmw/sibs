use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Block, Component, ElTarget, Element, VariableName},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct Each {
    pub variable: VariableName,
    pub input: Box<Element>,
    pub block: Block,
    pub token: usize,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Each);
        if reader.move_to().word(&[words::EACH]).is_some() {
            let (variable, input) = if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let variable = if let Some(Element::VariableName(variable, _)) =
                    Element::include(&mut inner, &[ElTarget::VariableName])?
                {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon.linked(&inner.token()?.id));
                    }
                    variable
                } else {
                    return Err(E::NoLoopVariable.linked(&inner.token()?.id));
                };
                let input = if let Some(el) =
                    Element::include(&mut inner, &[ElTarget::Function, ElTarget::VariableName])?
                {
                    Box::new(el)
                } else {
                    Err(E::NoLoopInput.by_reader(&inner))?
                };
                (variable, input)
            } else {
                return Err(E::NoLoopInitialization.linked(&reader.token()?.id));
            };
            let block = if let Some(Element::Block(block, _)) =
                Element::include(reader, &[ElTarget::Block])?
            {
                block
            } else {
                Err(E::NoGroup.by_reader(reader))?
            };
            Ok(Some(Each {
                input,
                variable,
                block,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "each({}; {}) {}", self.variable, self.input, self.block)
    }
}

impl Formation for Each {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}each({}; {}) {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.variable,
            self.input,
            self.block.format(cursor)
        )
    }
}

impl Operator for Each {
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
            let inputs = self
                .input
                .execute(
                    owner,
                    components,
                    args,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::NoInputForEach)?
                .as_strings()
                .ok_or(operator::E::FailConvertInputIntoStringsForEach)?;
            let mut output: Option<AnyValue> = None;
            let (loop_uuid, loop_token) = sc.open_loop().await?;
            for iteration in inputs.iter() {
                if loop_token.is_cancelled() {
                    break;
                }
                sc.set_var(&self.variable.name, AnyValue::new(iteration.to_string())?)
                    .await?;
                output = self
                    .block
                    .execute(
                        owner,
                        components,
                        args,
                        cx.clone(),
                        sc.clone(),
                        token.clone(),
                    )
                    .await?;
            }
            sc.close_loop(loop_uuid).await?;
            Ok(if output.is_none() {
                Some(AnyValue::empty())
            } else {
                output
            })
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Each,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/each.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Each::read(reader))? {
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
                while let Some(entity) = src.report_err_if(Each::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.variable.to_string()),
                        trim_carets(&reader.get_fragment(&entity.variable.token)?.lined),
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
                    assert!(Each::read(reader).is_err());
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
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/each.sibs"),
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
                    assert!(task
                        .execute(
                            None,
                            &[],
                            &[],
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new()
                        )
                        .await?
                        .is_some());
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
        elements::{task::Task, Block, Each, ElTarget, Element, VariableName},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Reader, Reading, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Each {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Block::arbitrary_with(deep),
                VariableName::arbitrary(),
                Element::arbitrary_with((vec![ElTarget::VariableName], deep)),
            )
                .prop_map(|(block, variable, input)| Each {
                    block,
                    variable,
                    input: Box::new(input),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(each: Each) {
        get_rt().block_on(async {
            let origin = format!("test {{\n{each};\n}};");
            read_string!(
                &Configuration::logs(false),
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
            args in any_with::<Each>(0)
        ) {
            reading(args.clone());
        }
    }
}
