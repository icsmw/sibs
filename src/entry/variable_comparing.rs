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
            let left = if let Some(el) =
                Element::include(&mut inner, &[ElTarget::VariableName, ElTarget::Function])?
            {
                Box::new(el)
            } else {
                restore(reader);
                return Ok(None);
            };
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
