use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter, TryExecute, Value,
        VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct First {
    pub block: Box<Element>,
    pub token: usize,
}

impl TryDissect<First> for First {
    fn try_dissect(reader: &mut Reader) -> Result<Option<First>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::First);
        if reader.move_to().word(&[words::FIRST]).is_some() {
            let Some(mut block) = Element::include(reader, &[ElTarget::Block])? else {
                return Err(E::NoFIRSTStatementBody.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElTarget::First);
            }
            Ok(Some(First {
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<First, First> for First {}

impl fmt::Display for First {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "first {}", self.block)
    }
}

impl Formation for First {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::First));
        format!(
            "{}first {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.block.format(&mut inner)
        )
    }
}

impl TokenGetter for First {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for First {
    fn varification<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }
    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move { self.block.linking(variables, owner, components, cx).await })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { self.block.expected(owner, components, cx).await })
    }
}

impl TryExecute for First {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [Value],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            self.block
                .execute(owner, components, args, cx, sc, token)
                .await
        })
    }
}

impl Execute for First {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::First,
        error::LinkedErr,
        inf::{tests::*, Configuration, TokenGetter},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/first.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(First::dissect(reader))? {
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
            &Configuration::logs(false),
            &include_str!("../../tests/reading/first.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(First::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token())?.lined),
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
        let samples = include_str!("../../tests/error/first.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(First::dissect(reader).is_err());
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
            operator::{Execute, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Dissect, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/first.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::dissect(reader))? {
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
        elements::{ElTarget, Element, First, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for First {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((vec![ElTarget::Block], deep))
                .prop_map(|block| First {
                    block: Box::new(block),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(first: First) {
        get_rt().block_on(async {
            let origin = format!("test {{\n{first};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(task) = src.report_err_if(Task::dissect(reader))? {
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
            args in any_with::<First>(0)
        ) {
            reading(args.clone());
        }
    }
}
