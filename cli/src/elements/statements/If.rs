use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
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
pub enum Thread {
    // (Subsequence, Block)
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

impl ExpectedValueType for Thread {
    fn varification<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            match self {
                Self::If(sub, bl) => {
                    sub.varification(owner, components, cx).await?;
                    bl.varification(owner, components, cx).await?;
                }
                Self::Else(bl) => {
                    bl.varification(owner, components, cx).await?;
                }
            };
            Ok(())
        })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            match self {
                Self::If(sub, bl) => {
                    sub.linking(variables, owner, components, cx).await?;
                    bl.linking(variables, owner, components, cx).await?;
                }
                Self::Else(bl) => {
                    bl.linking(variables, owner, components, cx).await?;
                }
            }
            Ok(())
        })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            match self {
                Self::If(_, block) => block.expected(owner, components, cx).await,
                Self::Else(block) => block.expected(owner, components, cx).await,
            }
        })
    }
}

impl TryExecute for Thread {
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
            match self {
                Self::If(subsequence, block) => {
                    if *subsequence
                        .execute(
                            owner,
                            components,
                            args,
                            cx.clone(),
                            sc.clone(),
                            token.clone(),
                        )
                        .await?
                        .ok_or(operator::E::NoResultFromProviso)?
                        .get::<bool>()
                        .ok_or(operator::E::NoBoolResultFromProviso)?
                    {
                        block.execute(owner, components, args, cx, sc, token).await
                    } else {
                        Ok(None)
                    }
                }
                Self::Else(block) => block.execute(owner, components, args, cx, sc, token).await,
            }
        })
    }
}

impl Execute for Thread {}

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
                cursor.offset_as_string_if(&[ElTarget::Block]),
                el.format(cursor),
                block.format(cursor)
            ),
            Self::Else(block) => format!(
                "{}else {}",
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

impl TryDissect<If> for If {
    fn try_dissect(reader: &mut Reader) -> Result<Option<If>, LinkedErr<E>> {
        let mut threads: Vec<Thread> = Vec::new();
        let close = reader.open_token(ElTarget::If);
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

impl TokenGetter for If {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for If {
    fn varification<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            for thr in self.threads.iter() {
                thr.varification(owner, components, cx).await?;
            }
            Ok(())
        })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            for thr in self.threads.iter() {
                thr.linking(variables, owner, components, cx).await?;
            }
            Ok(())
        })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            let mut refs: Option<ValueRef> = None;
            for value_ref in self.threads.iter() {
                if let Some(prev) = refs.as_ref() {
                    if prev != &value_ref.expected(owner, components, cx).await? {
                        return Err(operator::E::ReturnsDifferentTypes.by(self));
                    }
                } else {
                    refs = Some(value_ref.expected(owner, components, cx).await?);
                }
            }
            Ok(refs.unwrap_or(ValueRef::Empty))
        })
    }
}

impl TryExecute for If {
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
            for thread in self.threads.iter() {
                if let Some(output) = thread
                    .execute(
                        owner,
                        components,
                        args,
                        cx.clone(),
                        sc.clone(),
                        token.clone(),
                    )
                    .await?
                {
                    return Ok(Some(output));
                }
            }
            Ok(None)
        })
    }
}

impl Execute for If {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{If, Thread},
        error::LinkedErr,
        inf::{tests::*, Configuration, TokenGetter},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../tests/reading/if.sibs");
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
        let content = include_str!("../../tests/reading/if.sibs");
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
        let samples = include_str!("../../tests/error/if.sibs");
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
        let tasks_count = include_str!("../../tests/processing/if.sibs")
            .match_indices(chars::AT)
            .count();
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/if.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Task>, cx: Context, sc: Scope, _: Journal| async move {
                for (i, task) in tasks.iter().enumerate() {
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
                        .expect("if returns some value");
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
        elements::{task::Task, ElTarget, Element, If, Thread},
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

    fn reading(if_block: If) {
        get_rt().block_on(async {
            let origin = format!("test {{\n{if_block};\n}};");
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
            args in any_with::<If>(0)
        ) {
            reading(args.clone());
        }
    }
}
