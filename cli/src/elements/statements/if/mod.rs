pub mod condition;
pub mod subsequence;

pub use condition::*;
pub use subsequence::*;

use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Thread {
    // (IfSubsequence, Block)
    If(Element, Element),
    // Block
    Else(Element),
}

impl TokenGetter for Thread {
    fn token(&self) -> usize {
        match self {
            Self::If(el, _) => el.token(),
            Self::Else(block) => block.token(),
        }
    }
}

impl TryExpectedValueType for Thread {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            match self {
                Self::If(sub, bl) => {
                    sub.verification(owner, components, prev, cx).await?;
                    bl.verification(owner, components, prev, cx).await?;
                }
                Self::Else(bl) => {
                    bl.verification(owner, components, prev, cx).await?;
                }
            };
            Ok(())
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            match self {
                Self::If(sub, bl) => {
                    sub.linking(owner, components, prev, cx).await?;
                    bl.linking(owner, components, prev, cx).await?;
                }
                Self::Else(bl) => {
                    bl.linking(owner, components, prev, cx).await?;
                }
            }
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
        Box::pin(async move {
            match self {
                Self::If(_, block) => block.expected(owner, components, prev, cx).await,
                Self::Else(block) => block.expected(owner, components, prev, cx).await,
            }
        })
    }
}

impl TryExecute for Thread {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            match self {
                Self::If(subsequence, block) => {
                    if *subsequence
                        .execute(cx.clone())
                        .await?
                        .get::<bool>()
                        .ok_or(operator::E::NoBoolResultFromProviso)?
                    {
                        block.execute(cx).await
                    } else {
                        Ok(Value::empty())
                    }
                }
                Self::Else(block) => block.execute(cx).await,
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
                Self::If(el, block) => format!("if {el} {block}"),
                Self::Else(block) => format!("else {block}"),
            }
        )
    }
}

impl Formation for Thread {
    fn elements_count(&self) -> usize {
        match self {
            Self::If(el, _) => el.elements_count(),
            Self::Else(_) => 0,
        }
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        match self {
            Self::If(el, block) => format!(
                "{}if {} {}",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                el.format(cursor),
                block.format(cursor)
            ),
            Self::Else(block) => format!(
                "{}else {}",
                cursor.offset_as_string_if(&[ElementRef::Block]),
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

impl TryDissect<If> for If {
    fn try_dissect(reader: &mut Reader) -> Result<Option<If>, LinkedErr<E>> {
        let mut threads: Vec<Thread> = Vec::new();
        let close = reader.open_token(ElementRef::If);
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[words::IF]).is_some() {
                let conditions = Element::include(
                    reader,
                    &[ElementRef::IfSubsequence, ElementRef::IfCondition],
                )?
                .ok_or(E::NoConditionForIfStatement.by_reader(reader))?;
                let block = Element::include(reader, &[ElementRef::Block])?
                    .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
                threads.push(Thread::If(conditions, block));
            } else if reader.move_to().word(&[words::ELSE]).is_some() {
                if threads.is_empty() {
                    Err(E::NoMainBlockForIfStatement.by_reader(reader))?;
                }
                let block = Element::include(reader, &[ElementRef::Block])?
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

impl Dissect<If, If> for If {}

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
    fn elements_count(&self) -> usize {
        self.threads.iter().map(|th| th.elements_count()).sum()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::If));
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.threads
                .iter()
                .map(|el| el.format(&mut inner))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl TokenGetter for If {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for If {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for thr in self.threads.iter() {
                thr.try_verification(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            for thr in self.threads.iter() {
                thr.try_linking(owner, components, prev, cx).await?;
            }
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
        Box::pin(async move {
            let mut refs: Option<ValueRef> = None;
            for value_ref in self.threads.iter() {
                if let Some(prev_value) = refs.as_ref() {
                    if prev_value != &value_ref.try_expected(owner, components, prev, cx).await? {
                        return Err(operator::E::ReturnsDifferentTypes.by(self));
                    }
                } else {
                    refs = Some(value_ref.try_expected(owner, components, prev, cx).await?);
                }
            }
            Ok(refs.unwrap_or(ValueRef::Empty))
        })
    }
}

impl Processing for If {}

impl TryExecute for If {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            for thread in self.threads.iter() {
                let output = thread.try_execute(cx.clone()).await?;
                if !output.is_empty() {
                    return Ok(output);
                }
            }
            Ok(Value::empty())
        })
    }
}

#[cfg(test)]
use crate::elements::InnersGetter;

#[cfg(test)]
impl InnersGetter for If {
    fn get_inners(&self) -> Vec<&Element> {
        todo!("Switch threads to Element");
        Vec::new()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{If, Thread, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../../tests/reading/if.sibs");
        read_string!(
            &Configuration::logs(false),
            &content,
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(If::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, content.split('\n').count());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../../../tests/reading/if.sibs");
        read_string!(
            &Configuration::logs(false),
            &content,
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(If::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                        "Line: {}",
                        count + 1
                    );
                    for thr in entity.threads.iter() {
                        match thr {
                            Thread::If(el, block) => {
                                assert_eq!(
                                    trim_carets(&el.to_string()),
                                    trim_carets(&reader.get_fragment(&el.token())?.lined)
                                );
                                assert_eq!(
                                    trim_carets(&block.to_string()),
                                    trim_carets(&reader.get_fragment(&block.token())?.lined)
                                );
                            }
                            Thread::Else(block) => {
                                assert_eq!(
                                    trim_carets(&block.to_string()),
                                    trim_carets(&reader.get_fragment(&block.token())?.lined)
                                );
                            }
                        };
                    }
                    count += 1;
                }
                assert_eq!(count, content.split('\n').count());
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../../tests/error/if.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(If::dissect(reader).is_err());
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
            operator::{Execute, ExecuteContext, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        let tasks_count = include_str!("../../../tests/processing/if.sibs")
            .match_indices(chars::AT)
            .count();
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../../tests/processing/if.sibs"),
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
                for (i, task) in tasks.iter().enumerate() {
                    let result = task
                        .execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                        .await?;
                    assert_eq!(
                        result.as_string().expect("if returns string value"),
                        "true".to_owned(),
                        "Line: {}",
                        i + 1
                    );
                }
                assert_eq!(tasks_count, tasks.len());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{task::Task, Element, ElementRef, If, Thread},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Thread {
        type Parameters = (u8, usize);
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with((target, deep): Self::Parameters) -> Self::Strategy {
            if target == 0 {
                (
                    Element::arbitrary_with((
                        vec![ElementRef::IfSubsequence, ElementRef::IfCondition],
                        deep,
                    )),
                    Element::arbitrary_with((vec![ElementRef::Block], deep)),
                )
                    .prop_map(|(subsequence, block)| Thread::If(subsequence, block))
                    .boxed()
            } else {
                Element::arbitrary_with((vec![ElementRef::Block], deep))
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

    fn reading(if_block: If) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{if_block};\n}};");
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
            args in any_with::<If>(0)
        ) {
            reading(args.clone());
        }
    }
}
