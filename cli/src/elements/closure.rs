use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValueExpectation, Processing, Scope,
        TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Closure {
    pub args: Vec<Element>,
    pub block: Box<Element>,
    pub token: usize,
    pub uuid: Uuid,
}

impl Closure {
    pub fn get_vars_names(&self) -> Vec<String> {
        self.args
            .iter()
            .filter_map(|el| {
                if let Element::VariableName(el, _) = el {
                    Some(el.get_name())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
    }
    pub fn execute_block(
        &self,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            self.block
                .execute(ExecuteContext::unbound(cx, sc).token(token))
                .await
        })
    }
}

impl TryDissect<Closure> for Closure {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElementRef::Closure);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Ok(None);
        }
        let mut args_inner = reader.token()?.bound;
        let Some(block) = Element::include(reader, &[ElementRef::Block])? else {
            return Ok(None);
        };
        let mut args = Vec::new();
        while !args_inner.is_empty() {
            if let Some(el) = Element::include(&mut args_inner, &[ElementRef::VariableName])? {
                if args_inner.move_to().char(&[&chars::COMMA]).is_none() && !args_inner.is_empty() {
                    Err(E::MissedComma.by_reader(&args_inner))?;
                }
                args.push(el);
            } else {
                return Err(E::InvalidClosureArgument.by_reader(&args_inner));
            }
        }
        Ok(Some(Self {
            token: close(reader),
            block: Box::new(block),
            args,
            uuid: Uuid::new_v4(),
        }))
    }
}

impl Dissect<Closure, Closure> for Closure {}

impl fmt::Display for Closure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {}",
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.block,
        )
    }
}

impl Formation for Closure {
    fn elements_count(&self) -> usize {
        self.args.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let output = format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block, ElementRef::Component]),
            self
        );
        format!(
            "{output}{}",
            if cursor.parent.is_none() { ";\n" } else { "" }
        )
    }
}

impl TokenGetter for Closure {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Closure {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            // let parent = self
            //     .owner
            //     .as_ref()
            //     .ok_or(operator::E::ClosureIsNotBoundWithOwner)?;
            // for el in self.args.iter() {
            //     el.verification(owner, components, prev, cx).await?;
            // }
            // self.block.verification(owner, components, prev, cx).await?;
            // let desc = cx
            //     .get_func_desc(parent, prev.as_ref().map(|v| v.value.clone()).clone())
            //     .await?;
            // todo!("Not implemented");
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
            for el in self.args.iter() {
                el.linking(owner, components, prev, cx).await?;
            }
            self.block.linking(owner, components, prev, cx).await?;
            cx.closures.set(self.uuid, self.clone()).await?;
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::Closure) })
    }
}

impl Processing for Closure {}

impl TryExecute for Closure {
    fn try_execute<'a>(&'a self, _cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move { Ok(Value::Closure(self.uuid)) })
    }
}

#[cfg(test)]
mod proptest {
    use crate::elements::{Closure, Element, ElementRef};
    use proptest::prelude::*;
    use uuid::Uuid;

    impl Arbitrary for Closure {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElementRef::Block], deep)),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
                    0..=3,
                ),
            )
                .prop_map(|(block, args)| Closure {
                    args,
                    token: 0,
                    block: Box::new(block),
                    uuid: Uuid::new_v4(),
                })
                .boxed()
        }
    }

    // fn reading(func: Closure) {
    //     get_rt().block_on(async {
    //         let origin = format!("@test {{\n{func};\n}};");
    //         read_string!(
    //             &Configuration::logs(false),
    //             &origin,
    //             |reader: &mut Reader, src: &mut Sources| {
    //                 let task = src
    //                     .report_err_if(Task::dissect(reader))?
    //                     .expect("Task read");
    //                 assert_eq!(format!("{task};"), origin);
    //                 Ok::<(), LinkedErr<E>>(())
    //             }
    //         );
    //     })
    // }

    // proptest! {
    //     #![proptest_config(ProptestConfig {
    //         max_shrink_iters: 5000,
    //         ..ProptestConfig::with_cases(10)
    //     })]
    //     #[test]
    //     fn test_run_task(
    //         args in any_with::<Closure>(0)
    //     ) {
    //         reading(args.clone());
    //     }
    // }
}
