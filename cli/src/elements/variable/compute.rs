use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, LinkingResult, PrevValue, PrevValueExpectation, Scope,
        TokenGetter, TryExecute, TryExpectedValueType, Value, ValueRef, VerificationResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Operator {
    Inc,
    Dec,
    Div,
    Mlt,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dec => "-",
                Self::Div => "/",
                Self::Inc => "+",
                Self::Mlt => "*",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Compute {
    pub left: Box<Element>,
    pub operator: Operator,
    pub right: Box<Element>,
    pub token: usize,
}

impl TryDissect<Compute> for Compute {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Compute>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Compute);
        let Some(left) = Element::include(
            reader,
            &[
                ElTarget::VariableName,
                ElTarget::Function,
                ElTarget::If,
                ElTarget::Block,
                ElTarget::Integer,
            ],
        )?
        else {
            return Ok(None);
        };
        reader.move_to().any();
        let Some(operator) =
            reader
                .move_to()
                .char(&[&chars::INC, &chars::DEC, &chars::DIV, &chars::MLT])
        else {
            return Ok(None);
        };
        let operator = match operator {
            chars::INC => Operator::Inc,
            chars::DEC => Operator::Dec,
            chars::DIV => Operator::Div,
            chars::MLT => Operator::Mlt,
            _ => {
                return Err(E::UnknownOperator(operator.to_string()).by_reader(reader));
            }
        };
        let Some(right) = Element::include(
            reader,
            &[
                ElTarget::VariableName,
                ElTarget::Function,
                ElTarget::If,
                ElTarget::Block,
                ElTarget::Integer,
            ],
        )?
        else {
            return Err(E::NoRightSideAfterOperator.by_reader(reader));
        };
        Ok(Some(Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            token: close(reader),
        }))
    }
}

impl Dissect<Compute, Compute> for Compute {}

impl fmt::Display for Compute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl Formation for Compute {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Compute));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.left.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}

impl TokenGetter for Compute {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Compute {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.left.verification(owner, components, prev, cx).await?;
            self.right.verification(owner, components, prev, cx).await?;
            let left = self.left.expected(owner, components, prev, cx).await?;
            let right = self.right.expected(owner, components, prev, cx).await?;
            if !left.is_numeric() || !right.is_numeric() {
                Err(operator::E::ArithmeticWrongType.linked(&self.token))
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
            self.right.linking(owner, components, prev, cx).await
        })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move { Ok(ValueRef::isize) })
    }
}

impl TryExecute for Compute {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<PrevValue>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let left = self
                .left
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
                .as_num()
                .ok_or(operator::E::ArithmeticWrongType.by(&*self.left))?;
            let right = self
                .right
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
                .as_num()
                .ok_or(operator::E::ArithmeticWrongType.by(&*self.right))?;
            Ok(match self.operator {
                Operator::Inc => Value::isize(left + right),
                Operator::Dec => Value::isize(left - right),
                Operator::Div => Value::isize(left / right),
                Operator::Mlt => Value::isize(left * right),
            })
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{compute::Operator, Compute, ElTarget, Element, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Compute {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::VariableName, ElTarget::Integer]
                    } else {
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                        ]
                    },
                    deep,
                )),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::VariableName, ElTarget::Integer]
                    } else {
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                        ]
                    },
                    deep,
                )),
                prop_oneof![
                    Just(Operator::Div),
                    Just(Operator::Inc),
                    Just(Operator::Dec),
                    Just(Operator::Mlt)
                ]
                .boxed(),
            )
                .prop_map(move |(left, right, operator)| Compute {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(compute: Compute) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n$var = {compute};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    let task = src
                        .report_err_if(Task::dissect(reader))?
                        .expect("Task read");
                    assert_eq!(format!("{task};"), origin);
                    Ok::<(), LinkedErr<E>>(())
                }
            );
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_task(
            args in any_with::<Compute>(0)
        ) {
            reading(args.clone());
        }
    }
}
