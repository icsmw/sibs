use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Block, Component, Function, Reading, VariableName},
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Input {
    VariableName(VariableName),
    Function(Function),
}

impl Operator for Input {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::VariableName(v) => v.process(owner, components, args, cx),
                Self::Function(v) => v.process(owner, components, args, cx),
            }
            .await
        })
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Each {
    pub variable: VariableName,
    pub input: Input,
    pub block: Block,
    pub token: usize,
}

impl Reading<Each> for Each {
    fn read(reader: &mut Reader) -> Result<Option<Each>, E> {
        if reader.move_to().word(&[&words::EACH]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                if let Some(variable) = VariableName::read(&mut reader.token()?.bound)? {
                    if reader.until().char(&[&chars::OPEN_SQ_BRACKET]).is_some() {
                        let mut inner_reader = reader.token()?.bound;
                        let input =
                            if let Some(variable_name) = VariableName::read(&mut inner_reader)? {
                                Input::VariableName(variable_name)
                            } else if let Some(function) = Function::read(&mut inner_reader)? {
                                Input::Function(function)
                            } else {
                                Err(E::NoLoopInput)?
                            };
                        if reader
                            .group()
                            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                            .is_some()
                        {
                            let mut token = reader.token()?;
                            if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                                Err(E::MissedSemicolon)
                            } else {
                                Ok(Some(Each {
                                    variable,
                                    input,
                                    block: Block::read(&mut token.bound)?.ok_or(E::EmptyGroup)?,
                                    token: token.id,
                                }))
                            }
                        } else {
                            Err(E::NoGroup)
                        }
                    } else {
                        Err(E::NoGroup)
                    }
                } else {
                    Err(E::NoLoopVariable)
                }
            } else {
                Err(E::NoLoopVariable)
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Each {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EACH({}) {} {};", self.variable, self.input, self.block)
    }
}

impl Operator for Each {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let inputs = self
                .input
                .process(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoInputForEach)?
                .get_as_strings()
                .ok_or(operator::E::FailConvertInputIntoStringsForEach)?;
            let mut output: Option<AnyValue> = None;
            for iteration in inputs.iter() {
                cx.set_var(
                    self.variable.name.to_owned(),
                    AnyValue::new(iteration.to_string()),
                )
                .await;
                output = self.block.process(owner, components, args, cx).await?;
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
        inf::tests,
        reader::{
            entry::{Each, Reading, E},
            Reader,
        },
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../../../tests/reading/each.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Each::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&format!("{entity}"))
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../../tests/error/each.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
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
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{
            entry::{Reading, Task},
            Reader,
        },
    };
    const VALUES: &[(&str, &str)] = &[("a", "three"), ("b", "two"), ("c", "one")];

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::new(include_str!("../../../tests/processing/each.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            assert!(task.process(None, &[], &[], &mut cx).await?.is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.get_var(name).await.unwrap().get_as_string().unwrap(),
                value.to_string()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        inf::{operator::E, tests::*},
        reader::{
            entry::{
                block::Block,
                embedded::each::{Each, Input},
                function::Function,
                task::Task,
                variable_name::VariableName,
            },
            Reader, Reading,
        },
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Input {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec![VariableName::arbitrary()
                .prop_map(Input::VariableName)
                .boxed()];
            if permissions.func {
                allowed.push(
                    Function::arbitrary_with(scope.clone())
                        .prop_map(Input::Function)
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }

    impl Arbitrary for Each {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Each);
            let boxed = (
                Block::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
                Input::arbitrary_with(scope.clone()),
            )
                .prop_map(|(block, variable, input)| Each {
                    block,
                    variable,
                    input,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Each);
            boxed
        }
    }

    fn reading(each: Each) -> Result<(), E> {
        async_io::block_on(async {
            let origin = format!("test [\n{each}\n];");
            let mut reader = Reader::new(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(task.to_string(), origin);
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
