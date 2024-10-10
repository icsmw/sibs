use crate::{
    elements::{Cmb, Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValueExpectation, Processing, TryExecute,
        TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct IfSubsequence {
    pub subsequence: Vec<Element>,
    pub token: usize,
}

impl TryDissect<IfSubsequence> for IfSubsequence {
    fn try_dissect(reader: &mut Reader) -> Result<Option<IfSubsequence>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::IfSubsequence);
        let mut subsequence: Vec<Element> = Vec::new();
        while !reader.rest().trim().is_empty() {
            if subsequence.is_empty()
                || matches!(subsequence.last(), Some(Element::Combination(..)))
            {
                if let Some(el) = Element::include(
                    reader,
                    &[
                        ElementRef::Boolean,
                        ElementRef::Command,
                        ElementRef::Comparing,
                        ElementRef::Function,
                        ElementRef::VariableName,
                        ElementRef::Reference,
                        ElementRef::IfCondition,
                    ],
                )? {
                    subsequence.push(el);
                } else {
                    break;
                }
            } else if let Some(el) = Element::include(reader, &[ElementRef::Combination])? {
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
            Ok(Some(IfSubsequence {
                subsequence,
                token: close(reader),
            }))
        } else {
            println!(">>>>>>>>>>>>:{}", reader.rest());
            Err(E::FailToReadConditions.linked(&close(reader)))
        }
    }
}

impl Dissect<IfSubsequence, IfSubsequence> for IfSubsequence {}

impl fmt::Display for IfSubsequence {
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

impl Formation for IfSubsequence {
    fn elements_count(&self) -> usize {
        self.subsequence.iter().map(|s| s.elements_count()).sum()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.elements_count() > cursor.max_elements()
            || self.to_string().len() > cursor.max_len()
        {
            let mut inner = cursor.reown(Some(ElementRef::IfSubsequence));
            self.subsequence
                .chunks(2)
                .enumerate()
                .map(|(i, pair)| {
                    format!(
                        "{}{}{}",
                        if i == 0 {
                            cursor.offset_as_string_if(&[ElementRef::Block])
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
            format!("{}{self}", cursor.offset_as_string_if(&[ElementRef::Block]))
        }
    }
}

impl TokenGetter for IfSubsequence {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for IfSubsequence {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            for el in self.subsequence.iter() {
                el.verification(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }

    fn try_linking<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move {
            for el in self.subsequence.iter() {
                el.linking(owner, components, prev, cx).await?;
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
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::bool) })
    }
}

impl Processing for IfSubsequence {}

impl TryExecute for IfSubsequence {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let mut last_value = true;
            for el in self.subsequence.iter() {
                let value = el.execute(cx.clone()).await?;
                if let Some(cmb) = value.get::<Cmb>() {
                    match cmb {
                        Cmb::And => {
                            if !last_value {
                                return Ok(Value::bool(false));
                            }
                        }
                        Cmb::Or => {
                            if last_value {
                                return Ok(Value::bool(true));
                            }
                        }
                    }
                } else if let Some(value) = value.as_bool() {
                    last_value = value;
                } else {
                    Err(E::FailToParseValueOfSubsequenceElement)?;
                }
            }
            Ok(Value::bool(last_value))
        })
    }
}

#[cfg(test)]
use crate::elements::InnersGetter;

#[cfg(test)]
impl InnersGetter for IfSubsequence {
    fn get_inners(&self) -> Vec<&Element> {
        self.subsequence.iter().collect()
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{IfSubsequence, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(IfSubsequence::dissect(reader))?;
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
        let content = include_str!("../../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (count, str) in content.iter().enumerate() {
            read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity = src.report_err_if(IfSubsequence::dissect(reader))?;
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
        elements::{Element, ElementRef, IfSubsequence},
        inf::tests::MAX_DEEP,
    };
    use proptest::prelude::*;

    impl Arbitrary for IfSubsequence {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                prop::collection::vec(
                    Element::arbitrary_with((
                        if deep > MAX_DEEP {
                            vec![
                                ElementRef::Boolean,
                                ElementRef::Comparing,
                                ElementRef::Function,
                                ElementRef::VariableName,
                                ElementRef::Reference,
                            ]
                        } else {
                            vec![
                                ElementRef::Boolean,
                                ElementRef::Command,
                                ElementRef::Comparing,
                                ElementRef::Function,
                                ElementRef::VariableName,
                                ElementRef::Reference,
                                ElementRef::IfCondition,
                            ]
                        },
                        deep,
                    )),
                    1..=5,
                ),
                prop::collection::vec(
                    Element::arbitrary_with((vec![ElementRef::Combination], deep)),
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
                    IfSubsequence {
                        subsequence: result,
                        token: 0,
                    }
                })
                .boxed()
        }
    }
}
