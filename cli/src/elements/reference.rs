use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element, Gatekeeper},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, Formation, FormationCursor, Scope,
        TokenGetter, TryExecute,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::{
    cmp::{Eq, PartialEq},
    fmt,
};

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub struct Reference {
    pub path: Vec<String>,
    pub inputs: Vec<Element>,
    pub token: usize,
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Reference {}

impl TryDissect<Reference> for Reference {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Reference);
        if reader.move_to().char(&[&chars::COLON]).is_none() {
            return Ok(None);
        }
        let mut path: Vec<String> = Vec::new();
        let mut inputs: Vec<Element> = Vec::new();
        reader.trim();
        while let Some((content, stopped)) = reader.until().char(&[
            &chars::COLON,
            &chars::WS,
            &chars::OPEN_BRACKET,
            &chars::SEMICOLON,
            &chars::COMMA,
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
                let _ = inner.move_to().char(&[&chars::COMMA]);
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
    }
}

impl Dissect<Reference, Reference> for Reference {}

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

impl Formation for Reference {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Reference {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExecute for Reference {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let target = owner.ok_or(operator::E::NoOwnerComponent.by(self))?;
            let (parent, task) = if self.path.len() == 1 {
                (target, &self.path[0])
            } else if self.path.len() == 2 {
                let parent = if self.path[0] == SELF {
                    target
                } else {
                    components
                        .iter()
                        .find(|c| c.name.to_string() == self.path[0])
                        .ok_or(operator::E::NotFoundComponent(self.path[0].to_owned()).by(self))?
                };
                (parent, &self.path[1])
            } else {
                return Err(operator::E::InvalidPartsInReference.by(self));
            };
            let scope = cx
                .scope
                .create(format!("{}:{task}", parent.name), parent.get_cwd(&cx)?)
                .await?;
            let (task, gatekeepers) = parent.get_task(task).ok_or(
                operator::E::TaskNotFound(task.to_owned(), parent.name.to_string()).by(self),
            )?;
            let mut args: Vec<String> = Vec::new();
            for input in self.inputs.iter() {
                args.push(
                    input
                        .execute(
                            owner,
                            components,
                            inputs,
                            cx.clone(),
                            sc.clone(),
                            token.clone(),
                        )
                        .await?
                        .ok_or(operator::E::FailToGetAnyValueAsTaskArg.by(self))?
                        .as_string()
                        .ok_or(operator::E::FailToGetStringValue.by(self))?,
                );
            }
            let task_ref = task
                .as_reference(
                    owner,
                    components,
                    &args,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?;
            let skippable = Gatekeeper::skippable(
                gatekeepers,
                &task_ref,
                owner,
                components,
                cx.clone(),
                scope.clone(),
                token.clone(),
            )
            .await?;
            if skippable {
                cx.journal.debug(
                    task.get_name(),
                    format!("{task_ref} will be skipped because gatekeeper conclusion",),
                );
            }
            let result = if !skippable {
                task.execute(owner, components, &args, cx, scope.clone(), token)
                    .await
            } else {
                Ok(None)
            };
            scope.destroy().await?;
            result
        })
    }
}

impl Execute for Reference {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Reference,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/refs.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Reference::dissect(reader))? {
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
            &Configuration::logs(false),
            &include_str!("../tests/reading/refs.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Reference::dissect(reader))? {
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
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let result = Reference::dissect(reader);
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
