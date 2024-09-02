use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter, TryExecute, Value,
        ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Block {
    pub elements: Vec<Element>,
    pub owner: Option<ElTarget>,
    pub token: usize,
}

impl Block {
    pub fn set_owner(&mut self, owner: ElTarget) {
        self.owner = Some(owner);
    }
}

impl TryDissect<Block> for Block {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Block>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Block);
        if reader
            .group()
            .between(&chars::OPEN_CURLY_BRACE, &chars::CLOSE_CURLY_BRACE)
            .is_none()
        {
            return Ok(None);
        }
        let mut inner = reader.token()?.bound;
        let block_token_id = reader.token()?.id;
        let mut elements: Vec<Element> = Vec::new();
        loop {
            if let Some(el) = Element::exclude(
                &mut inner,
                &[
                    ElTarget::Block,
                    ElTarget::Task,
                    ElTarget::Component,
                    ElTarget::Condition,
                    ElTarget::Combination,
                    ElTarget::Subsequence,
                    ElTarget::VariableDeclaration,
                    ElTarget::VariableVariants,
                    ElTarget::VariableType,
                    ElTarget::SimpleString,
                    ElTarget::Gatekeeper,
                    ElTarget::Ppm,
                ],
            )? {
                if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    return if let Some((content, _)) = inner.until().char(&[&chars::SEMICOLON]) {
                        Err(E::UnrecognizedCode(content).by_reader(&inner))
                    } else {
                        Err(E::MissedSemicolon.by_reader(&inner))
                    };
                }
                elements.push(el);
                continue;
            }
            if inner.rest().trim().is_empty() {
                break if elements.is_empty() {
                    Err(E::EmptyBlock.linked(&block_token_id))
                } else {
                    Ok(Some(Block {
                        elements,
                        owner: None,
                        token: close(reader),
                    }))
                };
            } else {
                break Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner));
            }
        }
    }
}

impl Dissect<Block, Block> for Block {}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\n{}{}}}",
            self.elements
                .iter()
                .map(|el| format!("{el};",))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" }
        )
    }
}

impl Formation for Block {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Block)).right();
        format!(
            "{{\n{}{}{}}}",
            self.elements
                .iter()
                .map(|el| format!("{};", el.format(&mut inner),))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" },
            cursor.offset_as_string()
        )
    }
}

impl TokenGetter for Block {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Block {
    fn varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            for el in self.elements.iter() {
                el.varification(owner, components, cx).await?;
            }
            Ok(())
        })
    }
    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            for el in self.elements.iter() {
                el.linking(variables, owner, components, cx).await?;
            }
            Ok(())
        })
    }

    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move {
            let Some(el) = self.elements.last() else {
                return Ok(ValueRef::Empty);
            };
            el.expected(owner, components, cx).await
        })
    }
}

impl TryExecute for Block {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let mut output = Value::empty();
            for element in self.elements.iter() {
                output = element
                    .execute(
                        owner,
                        components,
                        args,
                        prev,
                        cx.clone(),
                        sc.clone(),
                        token.clone(),
                    )
                    .await?;
                if let (Some(ElTarget::First), false) = (self.owner.as_ref(), output.is_empty()) {
                    return Ok(output);
                }
            }
            Ok(output)
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Block,
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &format!(
                "{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}",
                include_str!("../tests/reading/if.sibs"),
                include_str!("../tests/reading/variable_assignation.sibs"),
                include_str!("../tests/reading/function.sibs"),
                include_str!("../tests/reading/optional.sibs"),
                include_str!("../tests/reading/each.sibs"),
                include_str!("../tests/reading/refs.sibs")
            ),
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(entity) = src.report_err_if(Block::dissect(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string())
                    );
                }
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &format!(
                "{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}\n{{{}}}",
                include_str!("../tests/reading/if.sibs"),
                include_str!("../tests/reading/variable_assignation.sibs"),
                include_str!("../tests/reading/function.sibs"),
                include_str!("../tests/reading/optional.sibs"),
                include_str!("../tests/reading/each.sibs"),
                include_str!("../tests/reading/refs.sibs")
            ),
            |reader: &mut Reader, src: &mut Sources| {
                while let Some(entity) = src.report_err_if(Block::dissect(reader))? {
                    assert_eq!(
                        trim_carets(&entity.to_string()),
                        reader.get_fragment(&entity.token)?.lined
                    );
                }
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Block, ElTarget, Element, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Block {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableAssignation,
                            ElTarget::Optional,
                            ElTarget::Command,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Boolean,
                            ElTarget::Integer,
                        ]
                    } else {
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableAssignation,
                            ElTarget::If,
                            ElTarget::Optional,
                            ElTarget::First,
                            ElTarget::Breaker,
                            ElTarget::Each,
                            ElTarget::Join,
                            ElTarget::Command,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Boolean,
                            ElTarget::Integer,
                        ]
                    },
                    deep,
                )),
                1..=10,
            )
            .prop_map(|elements| Block {
                elements,
                owner: None,
                token: 0,
            })
            .boxed()
        }
    }

    fn reading(block: Block) {
        get_rt().block_on(async {
            let origin = format!("@test {block};");
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
            args in any_with::<Block>(0)
        ) {
            reading(args.clone());
        }
    }
}
