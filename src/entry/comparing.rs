use crate::{
    entry::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Cmp {
    Equal,
    NotEqual,
}

impl fmt::Display for Cmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Equal => words::CMP_TRUE,
                Self::NotEqual => words::CMP_FALSE,
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
        if reader
            .until()
            .word(&[words::CMP_TRUE, words::CMP_FALSE])
            .is_some()
        {
            let mut inner = reader.token()?.bound;
            let left = if let Some(el) = Element::include(
                &mut inner,
                &[
                    ElTarget::VariableName,
                    ElTarget::Function,
                    ElTarget::PatternString,
                    ElTarget::Values,
                ],
            )? {
                Box::new(el)
            } else {
                restore(reader);
                return Ok(None);
            };
            if !inner.is_empty() {
                restore(reader);
                return Ok(None);
            }
            let cmp =
                if let Some(word) = reader.move_to().word(&[words::CMP_TRUE, words::CMP_FALSE]) {
                    if word == words::CMP_TRUE {
                        Cmp::Equal
                    } else {
                        Cmp::NotEqual
                    }
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
                    ElTarget::Values,
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
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Comparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.cmp, self.right)
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
                .ok_or(operator::E::NoResultFromLeftOnComparing)?
                .get_as_string()
                .ok_or(operator::E::FailToGetStringValue)?;
            let right = self
                .right
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoResultFromRightOnComparing)?
                .get_as_string()
                .ok_or(operator::E::FailToGetStringValue)?;
            Ok(Some(AnyValue::new(match self.cmp {
                Cmp::Equal => left == right,
                Cmp::NotEqual => left != right,
            })))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::Comparing,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/comparing.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Comparing::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 189);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/comparing.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Comparing::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
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
        assert_eq!(count, 189);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/comparing.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
    use crate::entry::{Cmp, Comparing, ElTarget, Element};
    use proptest::prelude::*;

    impl Arbitrary for Comparing {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with(vec![
                    ElTarget::VariableName,
                    ElTarget::Function,
                    ElTarget::PatternString,
                ]),
                Cmp::arbitrary(),
                Element::arbitrary_with(vec![
                    ElTarget::VariableName,
                    ElTarget::Function,
                    ElTarget::PatternString,
                ]),
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
