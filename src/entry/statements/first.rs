use crate::{
    entry::{Block, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        context::Context,
        operator::{Operator, OperatorPinnedResult},
    },
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
            let mut block = if let Some(Element::Block(block)) =
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
        error::LinkedErr,
        inf::{context::Context, tests::*},
        reader::{chars, Reader, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let cx: Context = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../../tests/reading/first.sibs").to_string(),
            &cx,
        );
        let mut count = 0;
        while let Some(entity) = report_if_err(&cx, First::read(&mut reader))? {
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
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../../tests/reading/first.sibs").to_string());
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
        reader::{chars, Reader, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../../tests/processing/first.sibs").to_string(),
            &cx,
        );
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
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
        entry::{Block, First, Task},
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
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

    fn reading(first: First) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{first};\n];");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
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
