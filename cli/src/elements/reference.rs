use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Element>,
    pub token: usize,
}

impl Reading<Reference> for Reference {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token();
        if reader.move_to().char(&[&chars::COLON]).is_some() {
            let mut path: Vec<String> = Vec::new();
            let mut inputs: Vec<Element> = Vec::new();
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
                while let Some(el) = Element::include(
                    &mut inner,
                    &[
                        ElTarget::VariableName,
                        ElTarget::Integer,
                        ElTarget::Boolean,
                        ElTarget::PatternString,
                    ],
                )? {
                    inputs.push(el);
                    let _ = inner.move_to().char(&[&chars::SEMICOLON]);
                }
                if !inner.is_empty() {
                    Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
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
                        .join("; ")
                )
            }
        )
    }
}

impl Formation for Reference {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
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
        cx: Context,
        sc: Scope,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let target = owner.ok_or(operator::E::NoOwnerComponent.by(self))?;
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
                            .ok_or(
                                operator::E::NotFoundComponent(self.path[0].to_owned()).by(self),
                            )?
                    },
                    &self.path[1],
                )
            } else {
                return Err(operator::E::InvalidPartsInReference.by(self));
            };
            let task = parent.get_task(task).ok_or(
                operator::E::TaskNotFound(task.to_owned(), parent.name.to_string()).by(self),
            )?;
            let mut args: Vec<String> = Vec::new();
            for input in self.inputs.iter() {
                args.push(
                    input
                        .execute(owner, components, inputs, cx.clone(), sc.clone())
                        .await?
                        .ok_or(operator::E::FailToGetAnyValueAsTaskArg.by(self))?
                        .get_as_string()
                        .ok_or(operator::E::FailToGetStringValue.by(self))?,
                );
            }
            task.execute(owner, components, &args, cx, sc).await
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Reference,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(),
            &include_str!("../tests/reading/refs.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Reference::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 6);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(),
            &include_str!("../tests/reading/refs.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Reference::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    for input in entity.inputs.iter() {
                        assert_eq!(
                            trim_carets(&input.to_string()),
                            trim_carets(&reader.get_fragment(&input.token())?.lined),
                            "Line: {}",
                            count + 1
                        );
                    }
                    count += 1;
                }
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/refs.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let result = Reference::read(reader);
                    assert!(result.is_err(), "Line: {}", count + 1);
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Reference},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Reference {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 2)
                    .prop_map(|path| Reference {
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
                        inputs: Vec::new(),
                        token: 0,
                    })
                    .boxed()
            } else {
                (
                    prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 2),
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![
                                ElTarget::VariableName,
                                ElTarget::Integer,
                                ElTarget::Boolean,
                                ElTarget::PatternString,
                            ],
                            deep,
                        )),
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
}
