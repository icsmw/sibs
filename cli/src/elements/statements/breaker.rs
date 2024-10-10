use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        Context, ExecuteContext, ExecutePinnedResult, ExpectedResult, Formation, FormationCursor,
        LinkingResult, PrevValueExpectation, Processing, TryExecute, TryExpectedValueType, Value,
        ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Breaker {
    pub token: usize,
}

impl TryDissect<Breaker> for Breaker {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Breaker>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Breaker);
        if reader.move_to().word(&[words::BREAK]).is_some() {
            Ok(Some(Breaker {
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Breaker, Breaker> for Breaker {}

impl fmt::Display for Breaker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", words::BREAK)
    }
}

impl Formation for Breaker {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::BREAK
        )
    }
}

impl TokenGetter for Breaker {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Breaker {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
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

impl Processing for Breaker {}

impl TryExecute for Breaker {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            cx.sc.break_loop().await?;
            Ok(Value::empty())
        })
    }
}

#[cfg(test)]
use crate::elements::InnersGetter;

#[cfg(test)]
impl InnersGetter for Breaker {
    fn get_inners(&self) -> Vec<&Element> {
        Vec::new()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Each, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/break.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Each::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};"))
                    );
                    count += 1;
                }
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/break.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Each::dissect(reader))? {
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
                assert_eq!(count, 3);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod processing {

    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };
    const VALUES: &[(&str, &str)] = &[("a", "two"), ("b", "one"), ("c", "one")];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/break.sibs"),
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
                for task in tasks.iter() {
                    task.execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
                        .await?;
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

    use crate::elements::Breaker;
    use proptest::prelude::*;

    impl Arbitrary for Breaker {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_deep: Self::Parameters) -> Self::Strategy {
            Just(Breaker { token: 0 }).boxed()
        }
    }
}
