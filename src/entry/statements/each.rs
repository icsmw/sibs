use crate::{
    entry::{Block, Component, ElTarget, Element, Function, VariableName},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;
#[derive(Debug, Clone)]
pub struct Each {
    pub variable: VariableName,
    pub input: Box<Element>,
    pub block: Block,
    pub token: usize,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().word(&[words::EACH]).is_some() {
            let (variable, input) = if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let variable = if let Some(Element::VariableName(variable)) =
                    Element::include(&mut inner, &[ElTarget::VariableName])?
                {
                    if inner.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon.linked(&inner.token()?.id));
                    }
                    variable
                } else {
                    return Err(E::NoLoopVariable.linked(&inner.token()?.id));
                };
                let input = if let Some(el) =
                    Element::include(&mut inner, &[ElTarget::Function, ElTarget::VariableName])?
                {
                    Box::new(el)
                } else {
                    Err(E::NoLoopInput.by_reader(&inner))?
                };
                (variable, input)
            } else {
                return Err(E::NoLoopInitialization.linked(&reader.token()?.id));
            };
            let block = if let Some(Element::Block(block)) =
                Element::include(reader, &[ElTarget::Block])?
            {
                block
            } else {
                Err(E::NoGroup.by_reader(reader))?
            };
            Ok(Some(Each {
                input,
                variable,
                block,
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EACH({}; {}) {}", self.variable, self.input, self.block)
    }
}

impl Operator for Each {
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
            let inputs = self
                .input
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoInputForEach)?
                .get_as_strings()
                .ok_or(operator::E::FailConvertInputIntoStringsForEach)?;
            let mut output: Option<AnyValue> = None;
            for iteration in inputs.iter() {
                cx.set_var(
                    self.variable.name.to_owned(),
                    AnyValue::new(iteration.to_string()),
                );
                output = self.block.execute(owner, components, args, cx).await?;
            }
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
        entry::Each,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../../tests/reading/each.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Each::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 7);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../../tests/reading/each.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Each::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined),
            );
            assert_eq!(
                tests::trim_carets(&entity.block.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.block.token)?.lined),
            );
            assert_eq!(
                tests::trim_carets(&entity.variable.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.variable.token)?.lined),
            );
            assert_eq!(
                tests::trim_carets(&entity.input.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.input.token())?.lined),
            );
            count += 1;
        }
        assert_eq!(count, 7);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/each.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(Each::read(&mut reader).is_err());
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
        reader::{Reader, Reading},
    };
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../../tests/processing/each.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            assert!(task.execute(None, &[], &[], &mut cx).await?.is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.get_var(name).unwrap().get_as_string().unwrap(),
                value.to_string()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        entry::{
            block::Block, element::Element, statements::each::Each, task::Task,
            variable::VariableName,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Each {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Each);
            let boxed = (
                Block::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
                Element::arbitrary_with(scope.clone()),
            )
                .prop_map(|(block, variable, input)| Each {
                    block,
                    variable,
                    input: Box::new(input),
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Each);
            boxed
        }
    }

    fn reading(each: Each) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{each};\n];");
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
            args in any_with::<Each>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
