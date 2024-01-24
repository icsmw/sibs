use crate::{
    cli,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator},
    },
    reader::{
        chars,
        entry::{
            Block, Component, Function, Reading, Reference, ValueString, VariableAssignation,
            VariableComparing,
        },
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub enum Action {
    VariableAssignation(VariableAssignation),
    ValueString(ValueString),
    Function(Function),
    Command(String),
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

#[derive(Debug)]
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

#[derive(Debug)]
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
                    if reader.until().char(&[&chars::SEMICOLON]).is_some() {
                        let mut token = reader.token()?;
                        if token.bound.contains().word(&[&words::DO_ON]) {
                            Err(E::NestedOptionalAction)?
                        }
                        reader.move_to().next();
                        Ok(Some(Optional {
                            token: token.id,
                            action: if let Some(assignation) =
                                VariableAssignation::read(&mut token.bound)?
                            {
                                Action::VariableAssignation(assignation)
                            } else if let Some(reference) = Reference::read(&mut token.bound)? {
                                Action::Reference(reference)
                            } else if let Some(func) = Function::read(&mut token.bound)? {
                                Action::Function(func)
                            } else if let Some(value_string) = ValueString::read(&mut token.bound)?
                            {
                                Action::ValueString(value_string)
                            } else if !token.bound.rest().trim().is_empty() {
                                Action::Command(token.bound.rest().trim().to_string())
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
    async fn process(
        &self,
        components: &[Component],
        args: &[String],
        cx: &mut Context,
    ) -> Result<Option<AnyValue>, operator::E> {
        Ok(None)
    }
}

#[cfg(test)]
mod test_optional {
    use crate::reader::{
        entry::{Optional, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 10);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/optional.sibs").to_string();
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
