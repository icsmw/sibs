use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Thread {
    If(Element, Element),
    Else(Element),
}

impl Operator for Thread {
    fn token(&self) -> usize {
        match self {
            Self::If(el, _) => el.token(),
            Self::Else(block) => block.token(),
        }
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::If(subsequence, block) => {
                    if *subsequence
                        .execute(owner, components, args, cx)
                        .await?
                        .ok_or(operator::E::NoResultFromProviso)?
                        .get_as::<bool>()
                        .ok_or(operator::E::NoBoolResultFromProviso)?
                    {
                        block.execute(owner, components, args, cx).await
                    } else {
                        Ok(None)
                    }
                }
                Self::Else(block) => block.execute(owner, components, args, cx).await,
            }
        })
    }
}

impl fmt::Display for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::If(el, block) => format!("IF {el} {block}"),
                Self::Else(block) => format!("ELSE {block}"),
            }
        )
    }
}

impl Formation for Thread {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        match self {
            Self::If(el, block) => format!(
                "{}IF {} {}",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                el.format(cursor),
                block.format(cursor)
            ),
            Self::Else(block) => format!(
                "{}ELSE {}",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                block.format(cursor)
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub threads: Vec<Thread>,
    pub token: usize,
}

impl Reading<If> for If {
    fn read(reader: &mut Reader) -> Result<Option<If>, LinkedErr<E>> {
        let mut threads: Vec<Thread> = vec![];
        let close = reader.open_token();
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[words::IF]).is_some() {
                let conditions =
                    Element::include(reader, &[ElTarget::Subsequence, ElTarget::Condition])?
                        .ok_or(E::NoConditionForIfStatement.by_reader(reader))?;
                let block = Element::include(reader, &[ElTarget::Block])?
                    .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
                threads.push(Thread::If(conditions, block));
            } else if reader.move_to().word(&[words::ELSE]).is_some() {
                if threads.is_empty() {
                    Err(E::NoMainBlockForIfStatement.by_reader(reader))?;
                }
                let block = Element::include(reader, &[ElTarget::Block])?
                    .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
                threads.push(Thread::Else(block));
            } else {
                break;
            }
        }
        if threads.is_empty() {
            Ok(None)
        } else {
            Ok(Some(If {
                threads,
                token: close(reader),
            }))
        }
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.threads
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Formation for If {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::If));
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.threads
                .iter()
                .map(|el| el.format(&mut inner))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Operator for If {
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
        Box::pin(async move {
            for thread in self.threads.iter() {
                if let Some(output) = thread.execute(owner, components, args, cx).await? {
                    return Ok(Some(output));
                }
            }
            Ok(None)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{If, Thread},
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/if.sibs");
        let mut reader = cx.reader().from_str(content);
        let mut count = 0;
        while let Some(entity) = tests::report_if_err(&cx, If::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};")),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, content.split('\n').count());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/if.sibs");
        let mut reader = cx.reader().from_str(content);
        let mut count = 0;
        while let Some(entity) = If::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined),
                "Line: {}",
                count + 1
            );
            for thr in entity.threads.iter() {
                match thr {
                    Thread::If(el, block) => {
                        assert_eq!(
                            tests::trim_carets(&el.to_string()),
                            tests::trim_carets(&reader.get_fragment(&el.token())?.lined)
                        );
                        assert_eq!(
                            tests::trim_carets(&block.to_string()),
                            tests::trim_carets(&reader.get_fragment(&block.token())?.lined)
                        );
                    }
                    Thread::Else(block) => {
                        assert_eq!(
                            tests::trim_carets(&block.to_string()),
                            tests::trim_carets(&reader.get_fragment(&block.token())?.lined)
                        );
                    }
                };
            }
            count += 1;
        }
        assert_eq!(count, content.split('\n').count());
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let samples = include_str!("../../tests/error/if.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
            assert!(If::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
            tests::report_if_err,
        },
        reader::{chars, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let tasks = include_str!("../../tests/processing/if.sibs")
            .match_indices("test [")
            .count();
        let mut reader = cx
            .reader()
            .from_str(include_str!("../../tests/processing/if.sibs"));
        let mut count = 0;
        while let Some(task) = report_if_err(&cx, Task::read(&mut reader))? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("IF returns some value");
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                result.get_as_string().expect("IF returns string value"),
                "true".to_owned(),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(tasks, count);
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{task::Task, ElTarget, Element, If, Thread},
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Thread {
        type Parameters = (u8, usize);
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((target, deep): Self::Parameters) -> Self::Strategy {
            if target == 0 {
                (
                    Element::arbitrary_with((
                        vec![ElTarget::Subsequence, ElTarget::Condition],
                        deep,
                    )),
                    Element::arbitrary_with((vec![ElTarget::Block], deep)),
                )
                    .prop_map(|(subsequence, block)| Thread::If(subsequence, block))
                    .boxed()
            } else {
                Element::arbitrary_with((vec![ElTarget::Block], deep))
                    .prop_map(Thread::Else)
                    .boxed()
            }
        }
    }

    impl Arbitrary for If {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(Thread::arbitrary_with((0, deep)), 1..=3),
                prop::collection::vec(Thread::arbitrary_with((1, deep)), 1..=1),
            )
                .prop_map(|(ifs, elses)| If {
                    threads: [ifs, elses].concat(),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(if_block: If) -> Result<(), E> {
        get_rt().block_on(async {
            let mut cx = Context::create().unbound()?;
            let origin = format!("test [\n{if_block};\n];");
            let mut reader = cx.reader().from_str(&origin);
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
            args in any_with::<If>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
