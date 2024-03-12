use crate::{
    entry::{Component, Element, ElementExd, SimpleString},
    error::LinkedErr,
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<ElementExd>,
    pub token: usize,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().char(&[&chars::COLON]).is_some() {
            let mut path: Vec<String> = vec![];
            let mut inputs: Vec<ElementExd> = vec![];
            reader.trim();
            while let Some((content, stopped)) = reader.until().char(&[
                &chars::COLON,
                &chars::WS,
                &chars::OPEN_BRACKET,
                &chars::SEMICOLON,
            ]) {
                if content.trim().is_empty() {
                    Err(E::EmptyPathToReference.by_reader(reader))?
                }
                path.push(content);
                if stopped != chars::COLON {
                    break;
                } else {
                    reader.move_to().next();
                    reader.trim();
                }
            }
            if !reader.rest().trim().is_empty()
                && Reader::is_ascii_alphabetic_and_alphanumeric(
                    reader.rest().trim(),
                    &[&chars::UNDERSCORE, &chars::DASH],
                )
            {
                path.push(reader.move_to().end());
            }
            if reader
                .group()
                .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
                .is_some()
            {
                let mut inner = reader.token()?.bound;
                let inputs_token_id = reader.token()?.id;
                while let Some(value) = inner
                    .until()
                    .char(&[&chars::COMMA])
                    .map(|(v, _)| {
                        inner.move_to().next();
                        v
                    })
                    .or_else(|| {
                        if inner.done() {
                            None
                        } else {
                            Some(inner.move_to().end())
                        }
                    })
                {
                    let mut inner = inner.token()?.bound;
                    inputs.push(if let Some(el) = Element::read(&mut inner)? {
                        match &el {
                            Element::Block(_)
                            | Element::Meta(_)
                            | Element::Reference(_)
                            | Element::Task(_)
                            | Element::Component(_) => {
                                return Err(E::InvalidArgumentForReference.linked(&el.token()))
                            }
                            _ => ElementExd::Element(el),
                        }
                    } else {
                        ElementExd::SimpleString(SimpleString {
                            value: value.trim().to_string(),
                            token: inner.token()?.id,
                        })
                    });
                }
                if inputs.is_empty() {
                    return Err(E::InvalidArgumentForReference.linked(&inputs_token_id));
                }
            }
            let token = close(reader);
            for part in path.iter() {
                if !Reader::is_ascii_alphabetic_and_alphanumeric(
                    part,
                    &[&chars::UNDERSCORE, &chars::DASH],
                ) {
                    Err(E::InvalidReference(part.to_owned()).linked(&token))?
                }
            }
            Ok(Some(Reference {
                token,
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
            ":{}{}",
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
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
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
                            .find(|c| c.name.to_string() == self.path[0])
                            .ok_or(operator::E::NotFoundComponent(self.path[0].to_owned()))?
                    },
                    &self.path[1],
                )
            } else {
                return Err(operator::E::InvalidPartsInReference);
            };
            let task = parent.get_task(task).ok_or(operator::E::TaskNotFound(
                task.to_owned(),
                parent.name.to_string(),
            ))?;
            let mut args: Vec<String> = vec![];
            for input in self.inputs.iter() {
                args.push(
                    input
                        .execute(owner, components, inputs, cx)
                        .await?
                        .ok_or(operator::E::FailToGetAnyValueAsTaskArg)?
                        .get_as_string()
                        .ok_or(operator::E::FailToGetStringValue)?,
                );
            }
            task.execute(owner, components, &args, cx).await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Reference,
        error::LinkedErr,
        inf::tests,
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(include_str!("../tests/reading/refs.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Reference::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 6);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/error/refs.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(Reference::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::entry::{ElTarget, ElementExd, Reference};
    use proptest::prelude::*;

    impl Arbitrary for Reference {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 2),
                prop::collection::vec(
                    ElementExd::arbitrary_with(vec![ElTarget::VariableName]),
                    0..5,
                ),
            )
                .prop_map(|(path, inputs)| Reference {
                    path: path
                        .iter()
                        .map(|p| {
                            if p.is_empty() {
                                "min".to_owned()
                            } else {
                                p.to_owned()
                            }
                        })
                        .collect::<Vec<String>>(),
                    inputs,
                    token: 0,
                })
                .boxed()
        }
    }
}
