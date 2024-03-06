use crate::{
    entry::{Block, Component, First, Function, PatternString, Values, VariableName},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Assignation {
    Function(Function),
    PatternString(PatternString),
    Values(Values),
    Block(Block),
    First(First),
}

impl fmt::Display for Assignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Assignation::PatternString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::Values(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            },
        )
    }
}

impl Operator for Assignation {
    fn token(&self) -> usize {
        match self {
            Assignation::PatternString(v) => v.token,
            Assignation::Block(v) => v.token,
            Assignation::Values(v) => v.token,
            Assignation::First(v) => v.token,
            Assignation::Function(v) => v.token,
        }
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Function(v) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v) => v.execute(owner, components, args, cx).await,
                Self::Values(v) => v.execute(owner, components, args, cx).await,
                Self::Block(v) => v.execute(owner, components, args, cx).await,
                Self::First(v) => v.execute(owner, components, args, cx).await,
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
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, LinkedErr<E>> {
        reader.state().set();
        //TODO: doesn't restore position if reads in function
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
                } else if let Some(value_string) = PatternString::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::PatternString(value_string),
                        token: close(reader),
                    }))
                } else {
                    Err(E::NoComparingOrAssignation.linked(&token.id))?
                }
            } else {
                Ok(None)
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
                Assignation::PatternString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::Values(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            },
        )
    }
}

impl Operator for VariableAssignation {
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
            let assignation = &self.assignation;
            let value = assignation
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoValueToAssign(self.name.name.clone()))?;
            cx.set_var(self.name.name.clone(), value);
            Ok(Some(AnyValue::new(())))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::VariableAssignation,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/variable_assignation.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 14);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/variable_assignation.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
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
    fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/error/variable_assignation.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a", "a"),
        ("b", "b"),
        ("c", "abc"),
        ("d", "ababc"),
        ("e", "ababc"),
        ("f", "\\{$a\\}\\{$b\\}\\{$c\\}"),
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::unbound(
            include_str!("../tests/processing/variable_assignation.sibs").to_string(),
        );
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
            block::Block,
            function::Function,
            pattern_string::PatternString,
            statements::first::First,
            task::Task,
            values::Values,
            variable_assignation::{Assignation, VariableAssignation},
            variable_name::VariableName,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Assignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec![PatternString::arbitrary_with(scope.clone())
                .prop_map(Self::PatternString)
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
        get_rt().block_on(async {
            let origin = format!("test [\n{assignation};\n];");
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
            args in any_with::<VariableAssignation>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
