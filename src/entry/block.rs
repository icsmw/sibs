use crate::{
    entry::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
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
                if let Some(el) = Element::exclude(&mut inner, &[ElTarget::Block])? {
                    if let (true, true) = (
                        !matches!(el, Element::Meta(_)),
                        inner.move_to().char(&[&chars::SEMICOLON]).is_none(),
                    ) {
                        return Err(E::MissedSemicolon.by_reader(&inner));
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
                .map(|el| format!(
                    "{el}{}",
                    if matches!(el, Element::Meta(_)) {
                        ""
                    } else {
                        ";"
                    }
                ))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" }
        )
    }
}

impl term::Display for Block {
    fn display(&self, term: &mut Term) {
        self.elements
            .iter()
            .filter(|el| matches!(el, Element::Meta(_)))
            .for_each(|el| el.display(term));
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
            // Block always should return some value.
            Ok(if output.is_none() {
                Some(AnyValue::new(()))
            } else {
                output
            })
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Block,
        error::LinkedErr,
        inf::tests,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ));

        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&entity.to_string())
            );
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(format!(
            "[{}]\n[{}]\n[{}]\n[{}]\n[{}]\n[{}]",
            include_str!("../tests/reading/if.sibs"),
            include_str!("../tests/reading/variable_assignation.sibs"),
            include_str!("../tests/reading/function.sibs"),
            include_str!("../tests/reading/optional.sibs"),
            include_str!("../tests/reading/each.sibs"),
            include_str!("../tests/reading/refs.sibs")
        ));
        while let Some(entity) = Block::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
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
        entry::{block::Block, element::Element, meta::Meta, task::Task},
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Block {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Block);
            let boxed = (
                prop::collection::vec(Element::arbitrary_with(scope.clone()), 1..=10),
                Meta::arbitrary_with(scope.clone()),
            )
                .prop_map(|(elements, meta)| Block {
                    elements,
                    owner: None,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Block);
            boxed
        }
    }

    fn reading(block: Block) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test {block};");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_run_task(
            args in any_with::<Block>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
