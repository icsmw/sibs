use tokio_util::sync::CancellationToken;

use crate::{
    elements::{string, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, spawner, Context, Execute, ExecutePinnedResult, ExpectedResult,
        TryExpectedValueType, Formation, FormationCursor, GlobalVariablesMap, LinkingResult,
        PrevValue, PrevValueExpectation, Scope, TokenGetter, TryExecute, Value, ValueRef,
        VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Command {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl TryDissect<Command> for Command {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Command>, LinkedErr<E>> {
        if let Some((_, elements, token)) = string::read(reader, chars::TILDA, ElTarget::Command)? {
            Ok(Some(Command { elements, token }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Command, Command> for Command {}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "`{}`",
            self.elements
                .iter()
                .map(|el| {
                    if let Element::SimpleString(el, _) = el {
                        el.to_string()
                    } else {
                        format!("{{{el}}}",)
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Formation for Command {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::PatternString));
        if self.to_string().len() > cursor.max_len()
            || self.elements.len() > cursor.max_inline_injections()
        {
            format!(
                "{}`{}`",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                self.elements
                    .iter()
                    .map(|el| {
                        if let Element::SimpleString(el, _) = el {
                            el.format(&mut inner)
                        } else {
                            format!(
                                "{{\n{}{}\n{}}}",
                                inner.right().offset_as_string(),
                                el.format(&mut inner.right()),
                                inner.offset_as_string()
                            )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )
        } else {
            format!("{}{self}", cursor.offset_as_string_if(&[ElTarget::Block]),)
        }
    }
}

impl TokenGetter for Command {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Command {
    fn try_varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            for el in self.elements.iter() {
                el.try_linking(variables, owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }

    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::Empty) })
    }
}

impl TryExecute for Command {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let mut command = String::new();
            for element in self.elements.iter() {
                if let Element::SimpleString(el, _) = element {
                    command.push_str(&el.to_string());
                } else {
                    command.push_str(
                        &element
                            .execute(
                                owner,
                                components,
                                args,
                                prev,
                                cx.clone(),
                                sc.clone(),
                                token.clone(),
                            )
                            .await?
                            .as_string()
                            .ok_or(operator::E::FailToGetValueAsString.by(self))?,
                    );
                }
            }
            let cwd = sc.get_cwd().await?.clone();
            match spawner::run(token, &command, &cwd, cx).await {
                Ok(None) => {
                    // Err(operator::E::SpawnedProcessCancelledError.by(self))
                    Ok(Value::Empty(()))
                }
                Ok(Some(status)) => {
                    if status.success() {
                        Ok(Value::Empty(()))
                    } else {
                        Err(operator::E::SpawnedProcessExitWithError.by(self))
                    }
                }
                Err(e) => Err(e)?,
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Command,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/command.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let origins = include_str!("../../tests/reading/command.sibs")
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Command::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "line {}",
                        count + 1
                    );
                    assert_eq!(
                        origins[count],
                        trim_carets(&format!("{entity};")),
                        "line {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 130);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/command.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Command::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&entity.to_string()),
                        reader.get_fragment(&entity.token)?.content
                    );
                    for el in entity.elements.iter() {
                        assert_eq!(
                            el.to_string().replace('\n', ""),
                            reader.get_fragment(&el.token())?.content
                        );
                    }
                    count += 1;
                }
                assert_eq!(count, 130);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Command, ElTarget, Element, Metadata, SimpleString},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Command {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                "[a-z][a-z0-9]*"
                    .prop_map(String::from)
                    .prop_map(|pattern| {
                        let pattern = if pattern.len() < 3 {
                            "min".to_owned()
                        } else {
                            pattern
                        };
                        Command {
                            elements: vec![Element::SimpleString(
                                SimpleString {
                                    value: pattern.clone(),
                                    token: 0,
                                },
                                Metadata::empty(),
                            )],
                            token: 0,
                        }
                    })
                    .boxed()
            } else {
                (
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![ElTarget::VariableName, ElTarget::Function, ElTarget::If],
                            deep,
                        )),
                        0..=2,
                    ),
                    prop::collection::vec(
                        Element::arbitrary_with((vec![ElTarget::SimpleString], deep)),
                        3,
                    ),
                )
                    .prop_map(|(injections, mut noise)| {
                        let mut elements: Vec<Element> = Vec::new();
                        for injection in injections.into_iter() {
                            elements.extend_from_slice(&[noise.remove(0), injection]);
                        }
                        elements.push(noise.remove(0));
                        Command { elements, token: 0 }
                    })
                    .boxed()
            }
        }
    }
}
