use tokio_util::sync::CancellationToken;

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
pub struct While {
    pub condition: Box<Element>,
    pub block: Box<Element>,
    pub token: usize,
}

impl TryDissect<While> for While {
    fn try_dissect(reader: &mut Reader) -> Result<Option<While>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::While);
        if reader.move_to().word(&[words::WHILE]).is_some() {
            let Some(condition) = Element::include(reader, &[ElementRef::Comparing])? else {
                return Err(E::NoConditionInWhile.by_reader(reader));
            };
            let Some(mut block) = Element::include(reader, &[ElementRef::Block])? else {
                return Err(E::NoBodyInForLoop.by_reader(reader));
            };
            if let Element::Block(block, _) = &mut block {
                block.set_owner(ElementRef::While);
                block.set_breaker(CancellationToken::new());
            }
            Ok(Some(Self {
                condition: Box::new(condition),
                block: Box::new(block),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<While, While> for While {}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", words::WHILE, self.condition, self.block)
    }
}

impl Formation for While {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::While));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::WHILE,
            self.condition,
            self.block.format(&mut inner)
        )
    }
}

impl TokenGetter for While {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for While {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.condition
                .verification(owner, components, prev, cx)
                .await?;
            self.block.verification(owner, components, prev, cx).await
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
            self.condition.linking(owner, components, prev, cx).await?;
            self.block.linking(owner, components, prev, cx).await
        })
    }

    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}

impl Processing for While {}

impl TryExecute for While {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let blk_token = if let Element::Block(el, _) = self.block.as_ref() {
                el.get_breaker()?
            } else {
                return Err(operator::E::BlockElementExpected.linked(&self.block.token()));
            };
            let (loop_uuid, loop_token) = cx.sc.open_loop(blk_token).await?;
            let mut n = u64::MIN;
            while n < u64::MAX {
                if loop_token.is_cancelled() {
                    break;
                }
                if !self
                    .condition
                    .execute(cx.clone())
                    .await?
                    .as_bool()
                    .ok_or(operator::E::ConditionReturnsNotBool.linked(&self.condition.token()))?
                {
                    break;
                }
                if n == u64::MAX - 1 {
                    cx.sc.close_loop(loop_uuid).await?;
                    return Err(operator::E::MaxIterations.linked(&self.token));
                }
                self.block.execute(cx.clone()).await?;
                n += 1;
            }
            cx.sc.close_loop(loop_uuid).await?;
            Ok(Value::Empty(()))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{TokenGetter, While},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/while.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(While::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 1);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/while.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(While::dissect(reader))? {
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
                assert_eq!(count, 1);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod processing {
    use crate::test_block;

    test_block!(
        simple,
        r#"
            $n = 0;
            while $n < 10 {
                $n += 1;
            };
            $n;
        "#,
        10isize
    );

    test_block!(
        with_break,
        r#"
            $n = 0;
            while $n < 10 {
                $n += 1;
                if $n == 5 {
                    break;
                };
            };
            $n;
        "#,
        5isize
    );
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Element, ElementRef, Task, While},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for While {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElementRef::Comparing], deep)),
                Element::arbitrary_with((vec![ElementRef::Block], deep)),
            )
                .prop_map(|(condition, block)| While {
                    condition: Box::new(condition),
                    block: Box::new(block),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(instance: While) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{instance};\n}};");
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
            args in any_with::<While>(0)
        ) {
            reading(args.clone());
        }
    }
}
