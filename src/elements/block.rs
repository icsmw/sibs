use crate::{
    elements::{Component, ElTarget, Element, Metadata},
    error::LinkedErr,
    inf::{
        term, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Term,
    },
    reader::{chars, Reader, Reading, E},
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

impl Reading<Block> for Block {
    fn read(reader: &mut Reader) -> Result<Option<Block>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader
            .group()
            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            let block_token_id = reader.token()?.id;
            let mut elements: Vec<Element> = vec![];
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
                    ],
                )? {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return if let Some((content, _)) = inner.until().char(&[&chars::SEMICOLON])
                        {
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
                    break Err(
                        E::UnrecognizedCode(inner.token()?.content.to_owned()).by_reader(&inner)
                    );
                }
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[\n{}{}]",
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
            "[\n{}{}{}]",
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

impl Operator for Block {
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
            let mut output: Option<AnyValue> = None;
            for element in self.elements.iter() {
                output = element.execute(owner, components, args, cx).await?;
                if let (Some(ElTarget::First), true) = (self.owner.as_ref(), output.is_some()) {
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
        inf::{context::Context, tests::*},
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx.reader().from_str(&format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ))?;
        while let Some(entity) = report_if_err(&mut cx, Block::read(&mut reader))? {
            assert_eq!(
                trim_carets(reader.recent()),
                trim_carets(&entity.to_string())
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx.reader().from_str(&format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ))?;
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.lined
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Block, ElTarget, Element, Task},
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Block {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                prop::collection::vec(
                    Element::arbitrary_with((
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableAssignation,
                            ElTarget::Optional,
                            ElTarget::Command,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Boolean,
                            ElTarget::Integer,
                        ],
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
            } else {
                prop::collection::vec(
                    Element::arbitrary_with((
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableAssignation,
                            ElTarget::If,
                            ElTarget::Optional,
                            ElTarget::First,
                            ElTarget::Each,
                            ElTarget::Command,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Boolean,
                            ElTarget::Integer,
                        ],
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
    }

    fn reading(block: Block) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test {block};");
            let mut cx: Context = Context::create().unbound()?;
            let mut reader = cx.reader().from_str(&origin)?;
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
            args in any_with::<Block>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
