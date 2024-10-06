use crate::{
    elements::{Element, ElementRef, TokenGetter},
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
pub struct Return {
    pub token: usize,
    pub output: Option<Box<Element>>,
}

impl TryDissect<Return> for Return {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Return>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::Return);
        if reader.move_to().word(&[words::RETURN]).is_none() {
            return Ok(None);
        }
        let output = if let Some(output) = Element::include(
            reader,
            &[
                ElementRef::Values,
                ElementRef::VariableName,
                ElementRef::Error,
                ElementRef::Function,
                ElementRef::If,
                ElementRef::Integer,
                ElementRef::Boolean,
                ElementRef::PatternString,
            ],
        )? {
            Some(Box::new(output))
        } else {
            let pin = reader.pin();
            let semicolon = reader.move_to().char(&[&chars::SEMICOLON]).is_some();
            pin(reader);
            if !semicolon {
                return Err(E::MissedReturnOutputOrMissedSemicolon.by_reader(reader));
            } else {
                None
            }
        };
        Ok(Some(Return {
            token: close(reader),
            output,
        }))
    }
}

impl Dissect<Return, Return> for Return {}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            words::RETURN,
            if let Some(el) = self.output.as_ref() {
                format!(" {el}")
            } else {
                String::new()
            }
        )
    }
}

impl Formation for Return {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::RETURN,
            if let Some(el) = self.output.as_ref() {
                format!(" {}", el.format(cursor))
            } else {
                String::new()
            }
        )
    }
}

impl TokenGetter for Return {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExpectedValueType for Return {
    fn try_verification<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move {
            if let Some(el) = self.output.as_ref() {
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
            if let Some(el) = self.output.as_ref() {
                el.linking(owner, components, prev, cx).await?;
            }
            Ok(())
        })
    }
    fn try_expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        prev: &'a Option<PrevValueExpectation>,
        cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            Ok(if let Some(el) = self.output.as_ref() {
                el.expected(owner, components, prev, cx).await?
            } else {
                ValueRef::Empty
            })
        })
    }
}

impl Processing for Return {}

impl TryExecute for Return {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            cx.sc
                .resolve(if let Some(el) = self.output.as_ref() {
                    el.execute(cx.clone()).await?
                } else {
                    Value::Empty(())
                })
                .await?;
            Ok(Value::Empty(()))
        })
    }
}

#[cfg(test)]
mod processing {
    use crate::{inf::Value, test_block};

    test_block!(
        returning,
        r#"
            return 5;
        "#,
        5isize
    );

    test_block!(
        returning_from_block,
        r#"
            $a = 13;
            return 5;
            13;
        "#,
        5isize
    );

    test_block!(
        returning_from_nested_block,
        r#"
            $a = 13;
            if $a == 13 {
                return 5;
            } else {
                false;
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_mt_nested_block,
        r#"
            $a = 13;
            if $a == 13 {
                if $a == 13 {
                    return 5;
                } else {
                    false;
                };
            } else {
                false;
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_loop,
        r#"
            for $n in 0..10 {
                if $n == 5 {
                    return 5;
                };
            };
            true;
        "#,
        5isize
    );

    test_block!(
        returning_from_loop_error,
        r#"
            for $n in 0..10 {
                if $n == 5 {
                    return Error "test";
                };
            };
            true;
        "#,
        Value::Error(String::from("test"))
    );
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{Element, ElementRef, Return, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Return {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            Element::arbitrary_with((
                if deep > MAX_DEEP {
                    vec![
                        ElementRef::VariableName,
                        ElementRef::Error,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                    ]
                } else {
                    vec![
                        ElementRef::Values,
                        ElementRef::VariableName,
                        ElementRef::Error,
                        ElementRef::Function,
                        ElementRef::If,
                        ElementRef::Integer,
                        ElementRef::Boolean,
                        ElementRef::PatternString,
                    ]
                },
                deep,
            ))
            .prop_map(|output| Return {
                output: Some(Box::new(output)),
                token: 0,
            })
            .boxed()
        }
    }

    fn reading(ret: Return) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{ret};\n}};");
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
            args in any_with::<Return>(0)
        ) {
            reading(args.clone());
        }
    }
}
