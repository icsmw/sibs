use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{
            Block, Component, First, Function, Reader, Reading, ValueString, Values, VariableName,
        },
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Assignation {
    Function(Function),
    ValueString(ValueString),
    Values(Values),
    Block(Block),
    First(First),
}

impl Assignation {
    pub fn token(&self) -> usize {
        match self {
            Assignation::ValueString(v) => v.token,
            Assignation::Block(v) => v.token,
            Assignation::Values(v) => v.token,
            Assignation::First(v) => v.token,
            Assignation::Function(v) => v.token,
        }
    }
}

impl fmt::Display for Assignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Assignation::ValueString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::Values(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            },
        )
    }
}

impl Operator for Assignation {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Function(v) => v.process(owner, components, args, cx).await,
                Self::ValueString(v) => v.process(owner, components, args, cx).await,
                Self::Values(v) => v.process(owner, components, args, cx).await,
                Self::Block(v) => v.process(owner, components, args, cx).await,
                Self::First(v) => v.process(owner, components, args, cx).await,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct VariableAssignation {
    pub name: VariableName,
    pub assignation: Assignation,
    pub token: usize,
}

impl Reading<VariableAssignation> for VariableAssignation {
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, E> {
        reader.state().set();
        let close = reader.open_token();
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to().char(&[&chars::EQUAL]).is_some() {
                if let Some(chars::EQUAL) = reader.next().char() {
                    // This is condition
                    reader.state().restore()?;
                    return Ok(None);
                }
                let assignation = if let Some(first) = First::read(reader)? {
                    Some(VariableAssignation {
                        name: name.clone(),
                        assignation: Assignation::First(first),
                        token: close(reader),
                    })
                } else if let Some(values) = Values::read(reader)? {
                    reader
                        .move_to()
                        .char(&[&chars::SEMICOLON])
                        .ok_or(E::MissedSemicolon)?;
                    Some(VariableAssignation {
                        name: name.clone(),
                        assignation: Assignation::Values(values),
                        token: close(reader),
                    })
                } else if let Some(block) = Block::read(reader)? {
                    reader
                        .move_to()
                        .char(&[&chars::SEMICOLON])
                        .ok_or(E::MissedSemicolon)?;
                    Some(VariableAssignation {
                        name: name.clone(),
                        assignation: Assignation::Block(block),
                        token: close(reader),
                    })
                } else {
                    None
                };
                if assignation.is_some() {
                    return Ok(assignation);
                }
                reader
                    .until()
                    .char(&[&chars::SEMICOLON])
                    .ok_or(E::MissedSemicolon)?;
                reader.move_to().next();
                let mut token = reader.token()?;
                if let Some(func) = Function::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::Function(func),
                        token: close(reader),
                    }))
                } else if let Some(value_string) = ValueString::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::ValueString(value_string),
                        token: close(reader),
                    }))
                } else {
                    Err(E::NoComparingOrAssignation)?
                }
            } else {
                Err(E::NoComparingOrAssignation)?
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for VariableAssignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = {}",
            self.name,
            match &self.assignation {
                Assignation::ValueString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::Values(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            },
        )
    }
}

impl Operator for VariableAssignation {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let assignation = &self.assignation;
            let value = assignation
                .process(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoValueToAssign(self.name.name.clone()))?;
            cx.set_var(self.name.name.clone(), value).await;
            Ok(Some(AnyValue::new(())))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        inf::tests,
        reader::{
            entry::{Reading, VariableAssignation},
            Reader, E,
        },
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("../../tests/reading/variable_assignation.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                reader.get_fragment(&entity.token)?.lined
            );
            assert_eq!(
                tests::trim_carets(&entity.name.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.name.token)?.content)
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.assignation.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.assignation.token())?.content
                ))
            );
            count += 1;
        }
        assert_eq!(count, 14);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/variable_assignation.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(VariableAssignation::read(&mut reader).is_err());
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

    const VALUES: &[(&str, &str)] = &[
        ("a", "a"),
        ("b", "b"),
        ("c", "abc"),
        ("d", "ababc"),
        ("e", "ababc"),
        ("f", "\\{$a\\}\\{$b\\}\\{$c\\}"),
    ];

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::new(
            include_str!("../../tests/processing/variable_assignation.sibs").to_string(),
        );
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
                embedded::first::First,
                function::Function,
                task::Task,
                value_strings::ValueString,
                values::Values,
                variable_assignation::{Assignation, VariableAssignation},
                variable_name::VariableName,
            },
            Reader, Reading,
        },
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Assignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec![ValueString::arbitrary_with(scope.clone())
                .prop_map(Self::ValueString)
                .boxed()];
            if permissions.func {
                allowed.push(
                    Function::arbitrary_with(scope.clone())
                        .prop_map(Self::Function)
                        .boxed(),
                );
            }
            if permissions.first {
                allowed.push(
                    First::arbitrary_with(scope.clone())
                        .prop_map(Self::First)
                        .boxed(),
                );
            }
            if permissions.block {
                allowed.push(
                    Block::arbitrary_with(scope.clone())
                        .prop_map(Self::Block)
                        .boxed(),
                );
            }
            if permissions.values {
                allowed.push(
                    Values::arbitrary_with(scope.clone())
                        .prop_map(Self::Values)
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }

    impl Arbitrary for VariableAssignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::VariableAssignation);
            let inner = scope.clone();
            let boxed = (
                Assignation::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
            )
                .prop_map(move |(assignation, name)| {
                    inner.write().unwrap().add_assignation(name.name.clone());
                    VariableAssignation {
                        assignation,
                        name,
                        token: 0,
                    }
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::VariableAssignation);
            boxed
        }
    }

    fn reading(assignation: VariableAssignation) -> Result<(), E> {
        async_io::block_on(async {
            let origin = format!("test [\n{assignation};\n];");
            let mut reader = Reader::new(origin.clone());
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
            args in any_with::<VariableAssignation>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
