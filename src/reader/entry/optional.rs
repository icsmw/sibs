use crate::{
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{
            Block, Command, Component, Function, Reading, Reference, ValueString,
            VariableAssignation, VariableComparing,
        },
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Action {
    VariableAssignation(VariableAssignation),
    ValueString(ValueString),
    Function(Function),
    Command(Command),
    Block(Block),
    Reference(Reference),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::VariableAssignation(v) => v.to_string(),
                Self::Reference(v) => v.to_string(),
                Self::Function(v) => format!("{v};"),
                Self::ValueString(v) => format!("{v};"),
                Self::Command(v) => format!("{v};"),
                Self::Block(v) => format!("{v};"),
            }
        )
    }
}

impl Operator for Action {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::VariableAssignation(v) => v.process(owner, components, args, cx).await,
                Self::Reference(v) => v.process(owner, components, args, cx).await,
                Self::Function(v) => v.process(owner, components, args, cx).await,
                Self::ValueString(v) => v.process(owner, components, args, cx).await,
                Self::Command(v) => v.process(owner, components, args, cx).await,
                Self::Block(v) => v.process(owner, components, args, cx).await,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum Condition {
    Function(Function),
    VariableComparing(VariableComparing),
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v) => v.to_string(),
                Self::VariableComparing(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Condition {
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
                Self::VariableComparing(v) => v.process(owner, components, args, cx).await,
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Optional {
    pub condition: Condition,
    pub action: Action,
    pub token: usize,
}

impl Reading<Optional> for Optional {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.state().set();
        if reader
            .move_to()
            .char(&[&chars::AT, &chars::DOLLAR])
            .is_some()
        {
            reader.state().restore()?;
            if reader
                .cancel_on(&[&chars::SEMICOLON, &chars::OPEN_SQ_BRACKET])
                .until()
                .word(&[words::DO_ON])
                .is_some()
            {
                let mut token = reader.token()?;
                let condition =
                    if let Some(variable_comparing) = VariableComparing::read(&mut token.bound)? {
                        Condition::VariableComparing(variable_comparing)
                    } else if let Some(function) = Function::read(&mut token.bound)? {
                        Condition::Function(function)
                    } else {
                        Err(E::NoFunctionOnOptionalAction)?
                    };
                if reader.move_to().word(&[&words::DO_ON]).is_some() {
                    if reader
                        .group()
                        .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                        .is_some()
                    {
                        let mut token = reader.token()?;
                        if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                            Err(E::MissedSemicolon)?
                        }
                        if let Some(block) = Block::read(&mut token.bound)? {
                            return Ok(Some(Optional {
                                token: token.id,
                                action: Action::Block(block),
                                condition,
                            }));
                        } else {
                            Err(E::InvalidBlock)?
                        }
                    }
                    if let Some(assignation) = VariableAssignation::read(reader)? {
                        return Ok(Some(Optional {
                            token: reader.token()?.id,
                            action: Action::VariableAssignation(assignation),
                            condition,
                        }));
                    }
                    if reader.until().char(&[&chars::SEMICOLON]).is_some() {
                        let mut token = reader.token()?;
                        if token.bound.contains().word(&[&words::DO_ON]) {
                            Err(E::NestedOptionalAction)?
                        }
                        reader.move_to().next();
                        Ok(Some(Optional {
                            token: token.id,
                            action: if let Some(reference) = Reference::read(&mut token.bound)? {
                                Action::Reference(reference)
                            } else if let Some(func) = Function::read(&mut token.bound)? {
                                Action::Function(func)
                            } else if let Some(value_string) = ValueString::read(&mut token.bound)?
                            {
                                Action::ValueString(value_string)
                            } else if !token.bound.rest().trim().is_empty() {
                                let cmd = token.bound.move_to().end();
                                let token = token.bound.token()?;
                                Action::Command(Command::new(cmd, token.id)?)
                            } else {
                                Err(E::NotActionForCondition)?
                            },
                            condition,
                        }))
                    } else {
                        Err(E::MissedSemicolon)?
                    }
                } else {
                    Err(E::FailParseOptionalAction)?
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.condition, self.action)
    }
}

impl Operator for Optional {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let condition = *self
                .condition
                .process(owner, components, args, cx)
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get_as::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            if !condition {
                Ok(None)
            } else {
                self.action.process(owner, components, args, cx).await
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::reader::{
        entry::{Optional, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../../tests/reading/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/optional.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Optional::read(&mut reader).is_err());
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

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::new(include_str!("../../tests/processing/optional.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .process(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
            assert_eq!(
                result.get_as_string().expect("Task returns string value"),
                "true".to_owned()
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
                command::Command,
                function::Function,
                optional::{Action, Condition, Optional},
                reference::Reference,
                task::Task,
                value_strings::ValueString,
                variable_assignation::VariableAssignation,
                variable_comparing::VariableComparing,
            },
            Reader, Reading,
        },
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Action {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Command::arbitrary_with(scope.clone()).prop_map(Action::Command),
                Block::arbitrary_with(scope.clone()).prop_map(Action::Block),
                VariableAssignation::arbitrary_with(scope.clone())
                    .prop_map(Action::VariableAssignation),
                ValueString::arbitrary_with(scope.clone()).prop_map(Action::ValueString),
                Reference::arbitrary_with(scope.clone()).prop_map(Action::Reference),
                Function::arbitrary_with(scope.clone()).prop_map(Action::Function),
            ]
            .boxed()
        }
    }

    impl Arbitrary for Condition {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                VariableComparing::arbitrary_with(scope.clone())
                    .prop_map(Condition::VariableComparing),
                Function::arbitrary_with(scope.clone()).prop_map(Condition::Function),
            ]
            .boxed()
        }
    }

    impl Arbitrary for Optional {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Optional);
            let boxed = (
                Condition::arbitrary_with(scope.clone()),
                Action::arbitrary_with(scope.clone()),
            )
                .prop_map(|(condition, action)| Optional {
                    condition,
                    action,
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Optional);
            boxed
        }
    }

    fn reading(optional: Optional) -> Result<(), E> {
        async_io::block_on(async {
            let origin = format!("test [\n{optional}\n];");
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
            args in any_with::<Optional>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
