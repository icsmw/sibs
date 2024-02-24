use crate::{
    entry::{
        Block, Command, Component, Function, PatternString, Reference, VariableAssignation,
        VariableComparing,
    },
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Action {
    VariableAssignation(VariableAssignation),
    PatternString(PatternString),
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
                Self::Function(v) => v.to_string(),
                Self::PatternString(v) => v.to_string(),
                Self::Command(v) => v.to_string(),
                Self::Block(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Action {
    fn token(&self) -> usize {
        match self {
            Self::VariableAssignation(v) => v.token,
            Self::Reference(v) => v.token,
            Self::Function(v) => v.token,
            Self::PatternString(v) => v.token,
            Self::Command(v) => v.token,
            Self::Block(v) => v.token,
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
                Self::VariableAssignation(v) => v.execute(owner, components, args, cx).await,
                Self::Reference(v) => v.execute(owner, components, args, cx).await,
                Self::Function(v) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v) => v.execute(owner, components, args, cx).await,
                Self::Command(v) => v.execute(owner, components, args, cx).await,
                Self::Block(v) => v.execute(owner, components, args, cx).await,
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
    fn token(&self) -> usize {
        match self {
            Self::Function(v) => v.token,
            Self::VariableComparing(v) => v.token,
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
                Self::VariableComparing(v) => v.execute(owner, components, args, cx).await,
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
        let close = reader.open_token();
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
                if reader.move_to().word(&[words::DO_ON]).is_some() {
                    if let Some(block) = Block::read(reader)? {
                        if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                            Err(E::MissedSemicolon)?
                        }
                        return Ok(Some(Optional {
                            token: close(reader),
                            action: Action::Block(block),
                            condition,
                        }));
                    }
                    if let Some(assignation) = VariableAssignation::read(reader)? {
                        return Ok(Some(Optional {
                            token: close(reader),
                            action: Action::VariableAssignation(assignation),
                            condition,
                        }));
                    }
                    let close_right_side = reader.open_token();
                    if reader.until().char(&[&chars::SEMICOLON]).is_some() {
                        reader.move_to().next_and_extend();
                        let mut token = reader.token()?;
                        if token.bound.contains().word(&[words::DO_ON]) {
                            Err(E::NestedOptionalAction)?
                        }
                        let action = if let Some(reference) = Reference::read(&mut token.bound)? {
                            Action::Reference(reference)
                        } else if let Some(func) = Function::read(&mut token.bound)? {
                            Action::Function(func)
                        } else if let Some(value_string) = PatternString::read(&mut token.bound)? {
                            Action::PatternString(value_string)
                        } else if !token.content.trim().is_empty() {
                            Action::Command(Command::new(token.content, close_right_side(reader))?)
                        } else {
                            Err(E::NotActionForCondition)?
                        };
                        Ok(Some(Optional {
                            token: close(reader),
                            action,
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
            let condition = *self
                .condition
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get_as::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            if !condition {
                Ok(None)
            } else {
                self.action.execute(owner, components, args, cx).await
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Optional,
        inf::{operator::Operator, tests},
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), E> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            assert_eq!(
                tests::trim_carets(&format!("{entity};")),
                reader.get_fragment(&entity.token)?.lined
            );
            // In some cases like with PatternString, semicolon can be skipped, because
            // belongs to parent entity (Optional).
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.action.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.action.token())?.lined
                )),
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.condition.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.condition.token())?.lined
                )),
            );
            count += 1;
        }
        assert_eq!(count, 11);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/optional.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/optional.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
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
        entry::{
            block::Block,
            command::Command,
            function::Function,
            optional::{Action, Condition, Optional},
            pattern_string::PatternString,
            reference::Reference,
            task::Task,
            variable_assignation::VariableAssignation,
            variable_comparing::VariableComparing,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
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
                PatternString::arbitrary_with(scope.clone()).prop_map(Action::PatternString),
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
            let origin = format!("test [\n{optional};\n];");
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
            args in any_with::<Optional>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
