use crate::{
    entry::{Cmp, Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
    },
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableComparing {
    pub left: Box<Element>,
    pub cmp: Cmp,
    pub right: Box<Element>,
    pub token: usize,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, LinkedErr<E>> {
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
            Ok(Some(VariableComparing {
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

impl fmt::Display for VariableComparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.cmp, self.right)
    }
}

impl Operator for VariableComparing {
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
            let left = self.left.execute(owner, components, args, cx).await?;
            let right = self.right.execute(owner, components, args, cx).await?;
            Ok(None)
            //TODO: finish implementation
            // Ok(Some(AnyValue::new(match self.cmp {
            //     Cmp::Equal => left == right,
            //     Cmp::NotEqual => left != right,
            // })))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::VariableComparing,
        error::LinkedErr,
        inf::tests,
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/comparing.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = VariableComparing::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 12);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    // #[test]
    // fn tokens() -> Result<(), LinkedErr<E>> {
    //     let mut reader =
    //         Reader::unbound(include_str!("../tests/reading/comparing.sibs").to_string());
    //     let mut count = 0;
    //     while let Some(entity) = VariableComparing::read(&mut reader)? {
    //         assert_eq!(
    //             tests::trim_carets(&format!("{entity};")),
    //             reader.get_fragment(&entity.token)?.lined
    //         );
    //         // In some cases like with PatternString, semicolon can be skipped, because
    //         // belongs to parent entity (VariableComparing).
    //         assert_eq!(
    //             tests::trim_semicolon(&tests::trim_carets(&entity.action.to_string())),
    //             tests::trim_semicolon(&tests::trim_carets(
    //                 &reader.get_fragment(&entity.action.token())?.lined
    //             )),
    //         );
    //         assert_eq!(
    //             tests::trim_semicolon(&tests::trim_carets(&entity.condition.to_string())),
    //             tests::trim_semicolon(&tests::trim_carets(
    //                 &reader.get_fragment(&entity.condition.token())?.lined
    //             )),
    //         );
    //         count += 1;
    //     }
    //     assert_eq!(count, 11);
    //     assert!(reader.rest().trim().is_empty());
    //     Ok(())
    // }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/optional.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(VariableComparing::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::{element::Element, statements::If::Cmp, variable_comparing::VariableComparing},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for VariableComparing {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with(scope.clone()),
                Cmp::arbitrary_with(scope.clone()),
                Element::arbitrary_with(scope.clone()),
            )
                .prop_map(|(left, cmp, right)| VariableComparing {
                    cmp,
                    left: Box::new(left),
                    right: Box::new(right),
                    token: 0,
                })
                .boxed()
        }
    }
}
