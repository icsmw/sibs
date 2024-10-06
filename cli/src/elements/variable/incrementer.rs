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
pub enum Operator {
    Inc,
    Dec,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dec => words::DEC_BY,
                Self::Inc => words::INC_BY,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Incrementer {
    pub variable: Box<Element>,
    pub operator: Operator,
    pub right: Box<Element>,
    pub token: usize,
}

impl TryDissect<Incrementer> for Incrementer {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Incrementer>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Incrementer);
        let Some(variable) = Element::include(reader, &[ElementRef::VariableName])? else {
            return Ok(None);
        };
        reader.move_to().any();
        let Some(operator) = reader.move_to().word_any(&[words::INC_BY, words::DEC_BY]) else {
            return Ok(None);
        };
        let operator = match operator.as_str() {
            words::INC_BY => Operator::Inc,
            words::DEC_BY => Operator::Dec,
            _ => {
                return Err(E::UnknownOperator(operator.to_string()).by_reader(reader));
            }
        };
        let Some(right) = Element::include(
            reader,
            &[
                ElementRef::VariableName,
                ElementRef::Function,
                ElementRef::If,
                ElementRef::Block,
                ElementRef::Integer,
            ],
        )?
        else {
            return Err(E::NoRightSideAfterOperator.by_reader(reader));
        };
        Ok(Some(Self {
            variable: Box::new(variable),
            operator,
            right: Box::new(right),
            token: close(reader),
        }))
    }
}

impl Dissect<Incrementer, Incrementer> for Incrementer {}

impl fmt::Display for Incrementer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.variable, self.operator, self.right)
    }
}

impl Formation for Incrementer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Incrementer));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.variable.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}

impl TokenGetter for Incrementer {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Incrementer {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            self.variable
                .verification(owner, components, prev, cx)
                .await?;
            self.right.verification(owner, components, prev, cx).await?;
            let variable = self.variable.expected(owner, components, prev, cx).await?;
            let right = self.right.expected(owner, components, prev, cx).await?;
            if !variable.is_numeric() || !right.is_numeric() {
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
            self.variable.linking(owner, components, prev, cx).await?;
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

impl Processing for Incrementer {}

impl TryExecute for Incrementer {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let name = if let Element::VariableName(el, _) = &*self.variable {
                el.get_name()
            } else {
                return Err(operator::E::NoVariableName.linked(&self.variable.token()));
            };
            let variable = self
                .variable
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(operator::E::ArithmeticWrongType.by(&*self.variable))?;
            let right = self
                .right
                .execute(cx.clone())
                .await?
                .as_num()
                .ok_or(operator::E::ArithmeticWrongType.by(&*self.right))?;
            let changed = Value::isize(match self.operator {
                Operator::Inc => variable + right,
                Operator::Dec => variable - right,
            });
            cx.sc.set_var(&name, changed.duplicate()).await?;
            Ok(changed)
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{incrementer::Operator, Element, ElementRef, Incrementer, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Incrementer {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElementRef::VariableName, ElementRef::Integer]
                    } else {
                        vec![
                            ElementRef::Function,
                            ElementRef::VariableName,
                            ElementRef::Integer,
                        ]
                    },
                    deep,
                )),
                prop_oneof![Just(Operator::Inc), Just(Operator::Dec),].boxed(),
            )
                .prop_map(move |(variable, right, operator)| Incrementer {
                    variable: Box::new(variable),
                    operator,
                    right: Box::new(right),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(incrementer: Incrementer) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{incrementer};\n}};");
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
            args in any_with::<Incrementer>(0)
        ) {
            reading(args.clone());
        }
    }
}
