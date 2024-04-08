use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
    },
    reader::{words, Reader, Reading, E},
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

impl Reading<Comparing> for Comparing {
    fn read(reader: &mut Reader) -> Result<Option<Comparing>, LinkedErr<E>> {
        let restore = reader.pin();
        let close = reader.open_token();
        let left = if let Some(el) = Element::include(
            reader,
            &[
                ElTarget::VariableName,
                ElTarget::Function,
                ElTarget::PatternString,
                ElTarget::Integer,
                ElTarget::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            restore(reader);
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
            restore(reader);
            return Ok(None);
        };
        let right = if let Some(el) = Element::include(
            reader,
            &[
                ElTarget::VariableName,
                ElTarget::Function,
                ElTarget::PatternString,
                ElTarget::Integer,
                ElTarget::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            restore(reader);
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

impl fmt::Display for Comparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.cmp, self.right)
    }
}

impl Formation for Comparing {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl Operator for Comparing {
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
            let left = self
                .left
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoResultFromLeftOnComparing)?;
            let right = self
                .right
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoResultFromRightOnComparing)?;
            Ok(Some(match self.cmp {
                Cmp::LeftBig | Cmp::RightBig => {
                    let left = left
                        .get_as_integer()
                        .ok_or(operator::E::FailToGetIntegerValue)?;
                    let right = right
                        .get_as_integer()
                        .ok_or(operator::E::FailToGetIntegerValue)?;
                    AnyValue::new(
                        (matches!(self.cmp, Cmp::LeftBig) && left > right)
                            || matches!(self.cmp, Cmp::RightBig) && left < right,
                    )
                }
                Cmp::LeftBigInc | Cmp::RightBigInc => {
                    let left = left
                        .get_as_integer()
                        .ok_or(operator::E::FailToGetIntegerValue)?;
                    let right = right
                        .get_as_integer()
                        .ok_or(operator::E::FailToGetIntegerValue)?;
                    AnyValue::new(
                        (matches!(self.cmp, Cmp::LeftBigInc) && left >= right)
                            || matches!(self.cmp, Cmp::RightBigInc) && left <= right,
                    )
                }
                _ => {
                    let left = left
                        .get_as_string()
                        .ok_or(operator::E::FailToGetStringValue)?;
                    let right = right
                        .get_as_string()
                        .ok_or(operator::E::FailToGetStringValue)?;
                    AnyValue::new(
                        (matches!(self.cmp, Cmp::Equal) && left == right)
                            || (matches!(self.cmp, Cmp::NotEqual) && left != right),
                    )
                }
            }))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Comparing,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            let mut reader = cx.reader().from_str(str)?;
            let entity = tests::report_if_err(&mut cx, Comparing::read(&mut reader))?;
            assert!(entity.is_some(), "Line: {}", count + 1);
            let entity = entity.unwrap();
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity}")),
                "Line: {}",
                count + 1
            );
            assert!(reader.rest().trim().is_empty(), "Line: {}", count + 1);
            count += 1;
        }
        assert_eq!(count, content.len());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let content = include_str!("../../tests/reading/comparing.sibs")
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut count = 0;
        for str in content.iter() {
            let mut reader = cx.reader().from_str(str)?;
            let entity = Comparing::read(&mut reader)?;
            assert!(entity.is_some(), "Line: {}", count + 1);
            let entity = entity.unwrap();
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                reader.get_fragment(&entity.token)?.lined
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.left.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.left.token())?.lined
                )),
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.right.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.right.token())?.lined
                )),
            );
            count += 1;
        }
        assert_eq!(count, content.len());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let samples = include_str!("../../tests/error/comparing.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample)?;
            let cmp = Comparing::read(&mut reader);
            assert!(cmp.is_err() || matches!(cmp, Ok(None)));
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{Cmp, Comparing, ElTarget, Element},
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
            if deep > MAX_DEEP {
                (
                    Element::arbitrary_with((
                        vec![ElTarget::VariableName, ElTarget::Integer, ElTarget::Boolean],
                        deep,
                    )),
                    Cmp::arbitrary(),
                    Element::arbitrary_with((
                        vec![ElTarget::VariableName, ElTarget::Integer, ElTarget::Boolean],
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
            } else {
                (
                    Element::arbitrary_with((
                        vec![
                            ElTarget::VariableName,
                            ElTarget::Function,
                            ElTarget::PatternString,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ],
                        deep,
                    )),
                    Cmp::arbitrary(),
                    Element::arbitrary_with((
                        vec![
                            ElTarget::VariableName,
                            ElTarget::Function,
                            ElTarget::PatternString,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ],
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
}
