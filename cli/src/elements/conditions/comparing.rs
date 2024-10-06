use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Cmp {
    Equal,
    NotEqual,
    LeftBig,
    RightBig,
    LeftBigInc,
    RightBigInc,
}

impl Cmp {
    pub fn from_str(value: &str) -> Result<Self, E> {
        match value {
            words::CMP_TRUE => Ok(Self::Equal),
            words::CMP_FALSE => Ok(Self::NotEqual),
            words::CMP_RBIG => Ok(Self::RightBig),
            words::CMP_LBIG => Ok(Self::LeftBig),
            words::CMP_LBIG_INC => Ok(Self::LeftBigInc),
            words::CMP_RBIG_INC => Ok(Self::RightBigInc),
            _ => Err(E::UnrecognizedCode(value.to_string())),
        }
    }
}

impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => words::CMP_TRUE,
                Self::NotEqual => words::CMP_FALSE,
                Self::LeftBig => words::CMP_LBIG,
                Self::RightBig => words::CMP_RBIG,
                Self::LeftBigInc => words::CMP_LBIG_INC,
                Self::RightBigInc => words::CMP_RBIG_INC,
            }
        )
    }
}
#[derive(Debug, Clone)]
pub struct Comparing {
    pub left: Box<Element>,
    pub cmp: Cmp,
    pub right: Box<Element>,
    pub token: usize,
}

impl TryDissect<Comparing> for Comparing {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Comparing>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Comparing);
        let left = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Function,
                ElementRef::PatternString,
                ElementRef::Integer,
                ElementRef::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        let cmp = if let Some(word) = reader.move_to().expression(&[
            words::CMP_TRUE,
            words::CMP_FALSE,
            words::CMP_LBIG_INC,
            words::CMP_RBIG_INC,
            words::CMP_LBIG,
            words::CMP_RBIG,
        ]) {
            Cmp::from_str(&word)?
        } else {
            return Ok(None);
        };
        let right = if let Some(el) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Function,
                ElementRef::PatternString,
                ElementRef::Integer,
                ElementRef::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        Ok(Some(Comparing {
            left,
            cmp,
            right,
            token: close(reader),
        }))
    }
}

impl Dissect<Comparing, Comparing> for Comparing {}

impl fmt::Display for Comparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.cmp, self.right)
    }
}

impl Formation for Comparing {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}

impl TokenGetter for Comparing {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Comparing {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            let left = self.left.expected(owner, components, prev, cx).await?;
            let right = self.right.expected(owner, components, prev, cx).await?;
            if !left.is_compatible(&right) {
                Err(operator::E::DismatchTypes(left, right).by(self))
            } else {
                Ok(())
            }
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
            self.left.linking(owner, components, prev, cx).await?;
            self.right.linking(owner, components, prev, cx).await?;
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

impl Processing for Comparing {}

impl TryExecute for Comparing {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let left = self.left.execute(cx.clone()).await?;
            let right = self.right.execute(cx).await?;
            Ok(match self.cmp {
                Cmp::LeftBig | Cmp::RightBig => {
                    let left = left.as_num().ok_or(operator::E::FailToGetIntegerValue)?;
                    let right = right.as_num().ok_or(operator::E::FailToGetIntegerValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::LeftBig) && left > right)
                            || matches!(self.cmp, Cmp::RightBig) && left < right,
                    )
                }
                Cmp::LeftBigInc | Cmp::RightBigInc => {
                    let left = left.as_num().ok_or(operator::E::FailToGetIntegerValue)?;
                    let right = right.as_num().ok_or(operator::E::FailToGetIntegerValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::LeftBigInc) && left >= right)
                            || matches!(self.cmp, Cmp::RightBigInc) && left <= right,
                    )
                }
                _ => {
                    // TODO: do not convert to string
                    let left = left.as_string().ok_or(operator::E::FailToGetStringValue)?;
                    let right = right.as_string().ok_or(operator::E::FailToGetStringValue)?;
                    Value::bool(
                        (matches!(self.cmp, Cmp::Equal) && left == right)
                            || (matches!(self.cmp, Cmp::NotEqual) && left != right),
                    )
                }
            })
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{Comparing, Element, ElementRef, TokenGetter},
        error::LinkedErr,
        inf::{tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let content = include_str!("../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity =
                        src.report_err_if(Element::include(reader, &[ElementRef::Comparing]))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity}")),
                        "Line: {}",
                        count + 1
                    );
                    assert!(reader.rest().trim().is_empty(), "Line: {}", count + 1);
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn tokens() {
        let content = include_str!("../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            count += read_string!(
                &Configuration::logs(false),
                str,
                |reader: &mut Reader, src: &mut Sources| {
                    let entity =
                        src.report_err_if(Element::include(reader, &[ElementRef::Comparing]))?;
                    assert!(entity.is_some(), "Line: {}", count + 1);
                    let entity = entity.unwrap();
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token())?.lined
                    );
                    if let Element::Comparing(entity, _) = entity {
                        assert_eq!(
                            trim_semicolon(&trim_carets(&entity.left.to_string())),
                            trim_semicolon(&trim_carets(&format!(
                                "{}{}",
                                if entity.left.get_metadata().inverting {
                                    chars::EXCLAMATION.to_string()
                                } else {
                                    String::new()
                                },
                                reader.get_fragment(&entity.left.token())?.lined
                            ))),
                        );
                        assert_eq!(
                            trim_semicolon(&trim_carets(&entity.right.to_string())),
                            trim_semicolon(&trim_carets(&format!(
                                "{}{}",
                                if entity.right.get_metadata().inverting {
                                    chars::EXCLAMATION.to_string()
                                } else {
                                    String::new()
                                },
                                reader.get_fragment(&entity.right.token())?.lined
                            ))),
                        );
                    } else {
                        panic!("Fail to extract Element::Comparing")
                    }
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, content.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/comparing.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let cmp = Comparing::dissect(reader);
                    assert!(cmp.is_err() || matches!(cmp, Ok(None)));
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
        elements::{Cmp, Comparing, Element, ElementRef},
        inf::tests::MAX_DEEP,
    };
    use proptest::prelude::*;

    impl Arbitrary for Cmp {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(Cmp::Equal),
                Just(Cmp::NotEqual),
                Just(Cmp::LeftBig),
                Just(Cmp::RightBig),
                Just(Cmp::LeftBigInc),
                Just(Cmp::RightBigInc)
            ]
            .boxed()
        }
    }

    impl Arbitrary for Comparing {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElementRef::VariableName,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                        ]
                    } else {
                        vec![
                            ElementRef::VariableName,
                            ElementRef::Function,
                            ElementRef::PatternString,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                        ]
                    },
                    deep,
                )),
                Cmp::arbitrary(),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElementRef::VariableName,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                        ]
                    } else {
                        vec![
                            ElementRef::VariableName,
                            ElementRef::Function,
                            ElementRef::PatternString,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                        ]
                    },
                    deep,
                )),
            )
                .prop_map(|(left, cmp, right)| Comparing {
                    cmp,
                    left: Box::new(left),
                    right: Box::new(right),
                    token: 0,
                })
                .boxed()
        }
    }
}
