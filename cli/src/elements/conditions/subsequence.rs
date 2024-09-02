use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Cmb, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter, TryExecute, Value,
        ValueRef, VerificationResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Subsequence {
    pub subsequence: Vec<Element>,
    pub token: usize,
}

impl TryDissect<Subsequence> for Subsequence {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Subsequence>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Subsequence);
        let mut subsequence: Vec<Element> = Vec::new();
        while !reader.rest().trim().is_empty() {
            if subsequence.is_empty()
                || matches!(subsequence.last(), Some(Element::Combination(..)))
            {
                if let Some(el) = Element::include(
                    reader,
                    &[
                        ElTarget::Boolean,
                        ElTarget::Command,
                        ElTarget::Comparing,
                        ElTarget::Function,
                        ElTarget::VariableName,
                        ElTarget::Reference,
                        ElTarget::Condition,
                    ],
                )? {
                    subsequence.push(el);
                } else {
                    break;
                }
            } else if let Some(el) = Element::include(reader, &[ElTarget::Combination])? {
                subsequence.push(el);
            } else {
                break;
            }
        }
        if subsequence.is_empty() {
            Ok(None)
        } else if reader.is_empty()
            || reader.next().is_word(&[
                words::IF,
                words::ELSE,
                &format!("{}", chars::OPEN_CURLY_BRACE),
            ])
        {
            Ok(Some(Subsequence {
                subsequence,
                token: close(reader),
            }))
        } else {
            Err(E::FailToReadConditions.linked(&close(reader)))
        }
    }
}

impl Dissect<Subsequence, Subsequence> for Subsequence {}

impl fmt::Display for Subsequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.subsequence
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Formation for Subsequence {
    fn elements_count(&self) -> usize {
        self.subsequence.iter().map(|s| s.elements_count()).sum()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.elements_count() > cursor.max_elements()
            || self.to_string().len() > cursor.max_len()
        {
            let mut inner = cursor.reown(Some(ElTarget::Subsequence));
            self.subsequence
                .chunks(2)
                .enumerate()
                .map(|(i, pair)| {
                    format!(
                        "{}{}{}",
                        if i == 0 {
                            cursor.offset_as_string_if(&[ElTarget::Block])
                        } else {
                            String::new()
                        },
                        pair[0].format(&mut inner),
                        if pair.len() > 1 {
                            format!(
                                "\n{}{}",
                                cursor.offset_as_string(),
                                pair[1].format(&mut inner)
                            )
                        } else {
                            String::new()
                        }
                    )
                })
                .collect::<Vec<String>>()
                .join("")
        } else {
            format!("{}{self}", cursor.offset_as_string_if(&[ElTarget::Block]))
        }
    }
}

impl TokenGetter for Subsequence {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Subsequence {
    fn varification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move {
            for el in self.subsequence.iter() {
                el.varification(owner, components, cx).await?;
            }
            Ok(())
        })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            for el in self.subsequence.iter() {
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
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl TryExecute for Subsequence {
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
            let mut last_value = true;
            for el in self.subsequence.iter() {
                let value = el
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
                    .ok_or(E::NoValueFromSubsequenceElement)?;
                if let Some(cmb) = value.get::<Cmb>() {
                    match cmb {
                        Cmb::And => {
                            if !last_value {
                                return Ok(Some(Value::bool(false)));
                            }
                        }
                        Cmb::Or => {
                            if last_value {
                                return Ok(Some(Value::bool(true)));
                            }
                        }
                    }
                } else if let Some(value) = value.as_bool() {
                    last_value = value;
                } else {
                    Err(E::FailToParseValueOfSubsequenceElement)?;
                }
            }
            Ok(Some(Value::bool(last_value)))
        })
    }
}


#[cfg(test)]
mod reading {
    use crate::{
        elements::Subsequence,
        error::LinkedErr,
        inf::{tests::*, Configuration, TokenGetter},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(Subsequence::dissect(reader))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(str),
                        trim_carets(&format!("{entity}")),
                        "Line: {}",
                        count + 1
                    );
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (count, str) in content.iter().enumerate() {
            read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(Subsequence::dissect(reader))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        trim_carets(&reader.get_fragment(&entity.token)?.lined),
                        "Line: {}",
                        count + 1
                    );
                    for el in entity.subsequence.iter() {
                        assert_eq!(
                            trim_carets(&format!("{el}")),
                            trim_carets(&reader.get_fragment(&el.token())?.lined),
                            "Line: {}",
                            count + 1
                        );
                    }
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        }
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{ElTarget, Element, Subsequence},
        inf::tests::MAX_DEEP,
    };
    use proptest::prelude::*;

    impl Arbitrary for Subsequence {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(
                    Element::arbitrary_with((
                        if deep > MAX_DEEP {
                            vec![
                                ElTarget::Boolean,
                                ElTarget::Comparing,
                                ElTarget::Function,
                                ElTarget::VariableName,
                                ElTarget::Reference,
                            ]
                        } else {
                            vec![
                                ElTarget::Boolean,
                                ElTarget::Command,
                                ElTarget::Comparing,
                                ElTarget::Function,
                                ElTarget::VariableName,
                                ElTarget::Reference,
                                ElTarget::Condition,
                            ]
                        },
                        deep,
                    )),
                    1..=5,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElTarget::Combination], deep)),
                    5..=5,
                ),
            )
                .prop_map(|(mut subsequences, mut combinations)| {
                    let mut result: Vec<Element> = Vec::new();
                    while let Some(subsequence) = subsequences.pop() {
                        result.push(subsequence);
                        result.push(combinations.pop().unwrap());
                    }
                    let _ = result.pop();
                    Subsequence {
                        subsequence: result,
                        token: 0,
                    }
                })
                .boxed()
        }
    }
}
