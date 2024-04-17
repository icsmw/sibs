use crate::{
    elements::{Block, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct First {
    pub block: Block,
    pub token: usize,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::FIRST]).is_some() {
            let mut block = if let Some(Element::Block(block, _)) =
                Element::include(reader, &[ElTarget::Block])?
            {
                block
            } else {
                return Err(E::NoFIRSTStatementBody.by_reader(reader));
            };
            block.set_owner(ElTarget::First);
            Ok(Some(First {
                block,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for First {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FIRST {}", self.block)
    }
}

impl Formation for First {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::First));
        format!(
            "{}FIRST {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.block.format(&mut inner)
        )
    }
}

impl Operator for First {
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
        Box::pin(async move { self.block.execute(owner, components, args, cx, sc).await })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::First,
        error::LinkedErr,
        inf::tests::*,
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        runner(
            include_str!("../../tests/reading/first.sibs"),
            |mut src, mut reader| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(First::read(&mut reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok(())
            },
        )
        .await
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        runner(
            include_str!("../../tests/reading/first.sibs"),
            |_, mut reader| {
                let mut count = 0;
                while let Some(entity) = First::read(&mut reader)? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                    );
                    assert_eq!(
                        trim_carets(&entity.block.to_string()),
                        trim_carets(&reader.get_fragment(&entity.block.token)?.lined),
                    );
                    count += 1;
                }
                assert_eq!(count, 2);
                assert!(reader.rest().trim().is_empty());
                Ok(())
            },
        )
        .await
    }

    #[tokio::test]
    async fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/error/first.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += runner(sample, |_, mut reader| {
                assert!(First::read(&mut reader).is_err());

                Ok(1)
            })
            .await?;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            tests::*,
        },
        reader::{chars, Reading, Sources},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let (tasks, src): (Vec<Task>, Sources) = runner(
            include_str!("../../tests/processing/first.sibs"),
            |src, mut reader| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = Task::read(&mut reader)? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<(Vec<Task>, Sources), LinkedErr<E>>((tasks, src))
            },
        )
        .await?;
        for task in tasks.iter() {
            let result = execution(&src, |cx, sc| {
                Box::pin(async move { task.execute(None, &[], &[], cx, sc).await })
            })
            .await?
            .expect("Task returns some value");
            assert_eq!(
                result.get_as_string().expect("Task returns string value"),
                "true".to_owned()
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Block, First, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for First {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Block::arbitrary_with(deep)
                .prop_map(|block| First { block, token: 0 })
                .boxed()
        }
    }

    fn reading(first: First) -> Result<(), LinkedErr<E>> {
        get_rt().block_on(async {
            let origin = format!("test [\n{first};\n];");
            runner(&origin, |_, mut reader| {
                while let Some(task) = Task::read(&mut reader)? {
                    assert_eq!(format!("{task};"), origin);
                }
                Ok::<(), LinkedErr<E>>(())
            })
            .await?;
            Ok(())
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
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
