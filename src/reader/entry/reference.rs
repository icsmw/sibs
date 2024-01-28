use crate::{
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Component, Reading, VariableName},
        Reader, E,
    },
};
use std::fmt;

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub enum Input {
    VariableName(VariableName),
    String(String),
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(s) => s.to_string(),
                Self::VariableName(v) => v.to_string(),
            }
        )
    }
}
impl Input {
    fn as_arg(&self, cx: &mut Context) -> Result<String, operator::E> {
        Ok(match self {
            Self::String(v) => v.to_owned(),
            Self::VariableName(name) => cx
                .get_var(&name.name)
                .ok_or(operator::E::VariableIsNotAssigned(name.name.to_owned()))?
                .get_as_string()
                .ok_or(operator::E::FailToGetStringValue)?,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Input>,
    pub token: usize,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        if reader.move_to().char(&[&chars::COLON]).is_some() {
            let mut path: Vec<String> = vec![];
            let mut inputs: Vec<Input> = vec![];
            while let Some((content, stopped_on)) = reader
                .until()
                .char(&[&chars::COLON, &chars::SEMICOLON])
                .map(|(content, stopped_on)| (content, Some(stopped_on)))
                .or_else(|| {
                    if reader.rest().trim().is_empty() {
                        None
                    } else {
                        Some((reader.move_to().end(), None))
                    }
                })
            {
                if content.trim().is_empty() {
                    Err(E::EmptyPathToReference)?
                }
                path.push(content);
                reader.move_to().next();
                if matches!(stopped_on, Some(chars::SEMICOLON)) {
                    break;
                }
            }
            if path.pop().is_some() {
                let mut token = reader.token()?;
                let name = token
                    .bound
                    .until()
                    .char(&[&chars::OPEN_BRACKET])
                    .map(|(value, _)| value)
                    .unwrap_or_else(|| token.bound.rest().to_string());
                if token
                    .bound
                    .group()
                    .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                    .is_some()
                {
                    let mut inner = token.bound.token()?.bound;
                    let mut last = false;
                    while let Some(value) = inner
                        .until()
                        .char(&[&chars::COMMA])
                        .map(|(v, _)| {
                            inner.move_to().next();
                            v
                        })
                        .or_else(|| {
                            if !last {
                                last = true;
                                Some(inner.move_to().end())
                            } else {
                                None
                            }
                        })
                    {
                        let mut value_reader = inner.token()?.bound;
                        inputs.push(
                            if let Some(variable_name) = VariableName::read(&mut value_reader)? {
                                Input::VariableName(variable_name)
                            } else {
                                Input::String(value.trim().to_string())
                            },
                        );
                    }
                }
                path.push(name);
            }
            for part in path.iter() {
                if !Reader::is_ascii_alphabetic_and_alphanumeric(
                    part,
                    &[&chars::UNDERSCORE, &chars::DASH],
                ) {
                    Err(E::InvalidReference)?
                }
            }
            Ok(Some(Reference {
                token: reader.token()?.id,
                path,
                inputs,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            ":{}{};",
            self.path.join(":"),
            if self.inputs.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.inputs
                        .iter()
                        .map(|input| input.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        )
    }
}

impl Operator for Reference {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        _: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let target = owner.ok_or(operator::E::NoOwnerComponent)?;
            let (parent, task) = if self.path.len() == 1 {
                (target, &self.path[0])
            } else if self.path.len() == 2 {
                (
                    if self.path[0] == SELF {
                        target
                    } else {
                        components
                            .iter()
                            .find(|c| c.name == self.path[0])
                            .ok_or(operator::E::NotFoundComponent(self.path[0].to_owned()))?
                    },
                    &self.path[1],
                )
            } else {
                return Err(operator::E::InvalidPartsInReference);
            };
            let task = parent.get_task(task).ok_or(operator::E::TaskNotFound(
                task.to_owned(),
                parent.name.to_owned(),
            ))?;
            let mut args: Vec<String> = vec![];
            for input in self.inputs.iter() {
                args.push(input.as_arg(cx)?);
            }
            task.process(owner, components, &args, cx).await
        })
    }
}

#[cfg(test)]
mod test_refs {
    use crate::reader::{
        entry::{Reading, Reference},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/refs.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Reference::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("./tests/error/refs.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Reference::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
