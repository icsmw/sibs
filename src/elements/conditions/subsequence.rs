use crate::{
    elements::{Cmb, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Subsequence {
    pub subsequence: Vec<Element>,
    pub token: usize,
}

impl Reading<Subsequence> for Subsequence {
    fn read(reader: &mut Reader) -> Result<Option<Subsequence>, LinkedErr<E>> {
        let close = reader.open_token();
        let mut subsequence: Vec<Element> = vec![];
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
                // &format!("{}", chars::SEMICOLON),
                &format!("{}", chars::OPEN_SQ_BRACKET),
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

impl Operator for Subsequence {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut last_value = true;
            for el in self.subsequence.iter() {
                let value = el
                    .execute(owner, components, args, cx)
                    .await?
                    .ok_or(E::NoValueFromSubsequenceElement)?;
                if let Some(cmb) = value.get_as::<Cmb>() {
                    match cmb {
                        Cmb::And => {
                            if !last_value {
                                return Ok(Some(AnyValue::new(false)));
                            }
                        }
                        Cmb::Or => {
                            if last_value {
                                return Ok(Some(AnyValue::new(true)));
                            }
                        }
                    }
                } else if let Some(value) = value.get_as_bool() {
                    last_value = value;
                } else {
                    Err(E::FailToParseValueOfSubsequenceElement)?;
                }
            }
            Ok(Some(AnyValue::new(last_value)))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Subsequence,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            let mut reader = cx.reader().from_str(str);
            let entity = tests::report_if_err(&cx, Subsequence::read(&mut reader))?;
            assert!(entity.is_some(), "Line: {}", count + 1);
            let entity = entity.unwrap();
            assert_eq!(
                tests::trim_carets(str),
                tests::trim_carets(&format!("{entity}")),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, content.len());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/subsequence.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for (count, str) in content.iter().enumerate() {
            let mut reader = cx.reader().from_str(str);
            let entity = Subsequence::read(&mut reader)?;
            assert!(entity.is_some(), "Line: {}", count + 1);
            let entity = entity.unwrap();
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                tests::trim_carets(&reader.get_fragment(&entity.token)?.lined),
                "Line: {}",
                count + 1
            );
            for el in entity.subsequence.iter() {
                assert_eq!(
                    tests::trim_carets(&format!("{el}")),
                    tests::trim_carets(&reader.get_fragment(&el.token())?.lined),
                    "Line: {}",
                    count + 1
                );
            }
        }
        Ok(())
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
            if deep > MAX_DEEP {
                (
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![
                                ElTarget::Boolean,
                                ElTarget::Comparing,
                                ElTarget::Function,
                                ElTarget::VariableName,
                                ElTarget::Reference,
                            ],
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
                        let mut result: Vec<Element> = vec![];
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
            } else {
                (
                    prop::collection::vec(
                        Element::arbitrary_with((
                            vec![
                                ElTarget::Boolean,
                                ElTarget::Command,
                                ElTarget::Comparing,
                                ElTarget::Function,
                                ElTarget::VariableName,
                                ElTarget::Reference,
                                ElTarget::Condition,
                            ],
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
                        let mut result: Vec<Element> = vec![];
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
}
