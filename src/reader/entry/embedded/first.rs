use crate::{
    inf::{
        context::Context,
        operator::{Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Block, Component, Reading},
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct First {
    pub block: Block,
    pub token: usize,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, E> {
        if reader.move_to().word(&[&words::FIRST]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut token = reader.token()?;
                if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    Err(E::MissedSemicolon)
                } else {
                    let mut block = Block::read(&mut token.bound)?.ok_or(E::EmptyGroup)?;
                    block.use_as_first();
                    Ok(Some(First {
                        block,
                        token: token.id,
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
        write!(f, "FIRST {};", self.block)
    }
}

impl Operator for First {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move { self.block.process(owner, components, args, cx).await })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        inf::tests,
        reader::{
            entry::{First, Reading},
            Reader, E,
        },
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../../../tests/reading/first.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = First::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../../tests/error/first.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
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
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{
            entry::{Reading, Task},
            Reader,
        },
    };

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::new(include_str!("../../../tests/processing/first.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .process(None, &[], &[], &mut cx)
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
        inf::{operator::E, tests::*},
        reader::{
            entry::{block::Block, embedded::first::First, task::Task},
            Reader, Reading,
        },
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
            let origin = format!("test [\n{first}\n];");
            let mut reader = Reader::new(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(task.to_string(), origin);
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
