use tokio_util::sync::CancellationToken;

use crate::{
    elements::{string, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter,
        TryExecute, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct PatternString {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl TryDissect<PatternString> for PatternString {
    fn try_dissect(reader: &mut Reader) -> Result<Option<PatternString>, LinkedErr<E>> {
        if let Some((_, elements, token)) =
            string::read(reader, chars::QUOTES, ElTarget::PatternString)?
        {
            Ok(Some(PatternString { elements, token }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<PatternString, PatternString> for PatternString {}

impl fmt::Display for PatternString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\"",
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

impl Formation for PatternString {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::PatternString));
        if self.to_string().len() > cursor.max_len()
            || self.elements.len() > cursor.max_inline_injections()
        {
            format!(
                "{}\"{}\"",
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

impl TokenGetter for PatternString {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for PatternString {
    fn varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            for el in self.elements.iter() {
                el.linking(variables, owner, components, cx).await?;
            }
            Ok(())
        })
    }

    fn expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { Ok(ValueRef::String) })
    }
}

impl TryExecute for PatternString {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let mut output = String::new();
            for element in self.elements.iter() {
                if let Element::SimpleString(el, _) = element {
                    output = format!("{output}{el}");
                } else {
                    output = format!(
                        "{output}{}",
                        element
                            .execute(
                                owner,
                                components,
                                args,
                                prev,
                                cx.clone(),
                                sc.clone(),
                                token.clone()
                            )
                            .await?
                            .ok_or(operator::E::FailToExtractValue)?
                            .as_string()
                            .ok_or(operator::E::FailToGetValueAsString)?
                    );
                }
            }
            Ok(Some(Value::String(output)))
        })
    }
}


#[cfg(test)]
mod reading {
    use crate::{
        elements::PatternString,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/pattern_string.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let origins = include_str!("../../tests/reading/pattern_string.sibs")
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut count = 0;
                while let Some(entity) = src.report_err_if(PatternString::dissect(reader))? {
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&entity.to_string()),
                    );
                    assert_eq!(
                        origins[count],
                        trim_carets(&entity.to_string()),
                        "line {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 96);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/pattern_string.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(PatternString::dissect(reader))? {
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
                assert_eq!(count, 96);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{ElTarget, Element, Metadata, PatternString, SimpleString},
        inf::tests::MAX_DEEP,
    };
    use proptest::prelude::*;

    impl Arbitrary for PatternString {
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
                        PatternString {
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
                        PatternString { elements, token: 0 }
                    })
                    .boxed()
            }
        }
    }
}
