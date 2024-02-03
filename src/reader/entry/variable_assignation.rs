use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Block, Component, First, Function, Reader, Reading, ValueString, VariableName},
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Assignation {
    Function(Function),
    ValueString(ValueString),
    Block(Block),
    First(First),
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
                        token: reader.token()?.id,
                    })
                } else if reader
                    .group()
                    .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                    .is_some()
                {
                    let mut group_token = reader.token()?;
                    if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                        return Err(E::MissedSemicolon)?;
                    } else {
                        Some(VariableAssignation {
                            name: name.clone(),
                            assignation: Assignation::Block(
                                Block::read(&mut group_token.bound)?.ok_or(E::EmptyGroup)?,
                            ),
                            token: group_token.id,
                        })
                    }
                } else {
                    None
                };
                if assignation.is_some() {
                    reader.move_to().next();
                    return Ok(assignation);
                }
                let mut token = reader
                    .until()
                    .char(&[&chars::SEMICOLON])
                    .map(|_| {
                        reader.move_to().next();
                        reader.token()
                    })
                    .unwrap_or_else(|| {
                        let _ = reader.move_to().end();
                        reader.token()
                    })?;
                if let Some(func) = Function::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::Function(func),
                        token: token.id,
                    }))
                } else if let Some(value_string) = ValueString::read(&mut token.bound)? {
                    Ok(Some(VariableAssignation {
                        name,
                        assignation: Assignation::ValueString(value_string),
                        token: token.id,
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
            "{} = {}{}",
            self.name,
            match &self.assignation {
                Assignation::ValueString(v) => v.to_string(),
                Assignation::Block(v) => v.to_string(),
                Assignation::First(v) => v.to_string(),
                Assignation::Function(v) => v.to_string(),
            },
            match &self.assignation {
                Assignation::First(_) => "",
                _ => ";",
            }
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
mod test_variable_assignation {
    use crate::reader::{
        entry::{Reading, VariableAssignation},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader =
            Reader::new(include_str!("../../tests/normal/variable_assignation.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
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
mod proptest {
    use crate::{
        inf::tests::*,
        reader::entry::{
            function::Function,
            value_strings::ValueString,
            variable_assignation::{Assignation, VariableAssignation},
            variable_name::VariableName,
        },
    };
    use proptest::prelude::*;

    impl Arbitrary for Assignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Function::arbitrary_with(scope.clone()).prop_map(Self::Function),
                ValueString::arbitrary_with(scope.clone()).prop_map(Self::ValueString),
            ]
            .boxed()
        }
    }

    impl Arbitrary for VariableAssignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                Assignation::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
            )
                .prop_map(move |(assignation, name)| {
                    scope.write().unwrap().add_assignation(name.name.clone());
                    VariableAssignation {
                        assignation,
                        name,
                        token: 0,
                    }
                })
                .boxed()
        }
    }
}
