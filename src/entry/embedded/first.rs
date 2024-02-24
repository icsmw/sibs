use crate::{
    entry::{Block, Component},
    inf::{
        context::Context,
        operator::{Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct First {
    pub block: Block,
    pub token: usize,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, E> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::FIRST]).is_some() {
            if let Some(mut block) = Block::read(reader)? {
                if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    Err(E::MissedSemicolon)
                } else {
                    block.use_as_first();
                    Ok(Some(First {
                        block,
                        token: close(reader),
                    }))
                }
            } else {
                Err(E::NoGroup)
            }
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

impl Operator for First {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move { self.block.execute(owner, components, args, cx).await })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::First,
        inf::tests,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::unbound(include_str!("../../tests/reading/first.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = First::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), E> {
        let mut reader =
            Reader::unbound(include_str!("../../tests/reading/first.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = First::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined),
            );
            assert_eq!(
                tests::trim_carets(&entity.block.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.block.token)?.lined),
            );
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/first.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(First::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../../tests/processing/first.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
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
        entry::{block::Block, embedded::first::First, task::Task},
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for First {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::First);
            let boxed = Block::arbitrary_with(scope.clone())
                .prop_map(|block| First { block, token: 0 })
                .boxed();
            scope.write().unwrap().exclude(Entity::First);
            boxed
        }
    }

    fn reading(first: First) -> Result<(), E> {
        async_io::block_on(async {
            let origin = format!("test [\n{first};\n];");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_run_task(
            args in any_with::<First>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
