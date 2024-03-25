use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
    reader::{words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {
    pub condition: Box<Element>,
    pub action: Box<Element>,
    pub token: usize,
}

impl Reading<Optional> for Optional {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(None);
        }
        let restore = reader.pin();
        let close = reader.open_token();
        let condition = if let Some(el) = Element::include(
            reader,
            &[
                ElTarget::Function,
                ElTarget::VariableName,
                ElTarget::Block,
                ElTarget::Reference,
                ElTarget::Comparing,
            ],
        )? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        if !reader.rest().trim().starts_with(words::DO_ON) {
            restore(reader);
            return Ok(None);
        }
        if reader.move_to().expression(&[words::DO_ON]).is_none() {
            return Err(E::NoOptionalRedirection.by_reader(reader));
        }
        let action = if let Some(el) = Element::include(
            reader,
            &[
                ElTarget::Function,
                ElTarget::Reference,
                ElTarget::VariableAssignation,
                ElTarget::VariableName,
                ElTarget::Block,
                ElTarget::Each,
                ElTarget::First,
                ElTarget::PatternString,
                ElTarget::Command,
                ElTarget::Integer,
                ElTarget::Boolean,
            ],
        )? {
            Box::new(el)
        } else {
            return Err(E::FailFindActionForOptional.by_reader(reader));
        };
        Ok(Some(Optional {
            token: close(reader),
            action,
            condition,
        }))
    }
}

impl fmt::Display for Optional {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} => {}", self.condition, self.action)
    }
}

impl Formation for Optional {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::Optional));
        format!(
            "{}{} => {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.condition.format(&mut inner),
            self.action.format(&mut inner),
        )
        // format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl Operator for Optional {
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
            let condition = *self
                .condition
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get_as::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            if !condition {
                Ok(None)
            } else {
                self.action.execute(owner, components, args, cx).await
            }
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Optional,
        error::LinkedErr,
        inf::{context::Context, operator::Operator, tests},
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/reading/optional.sibs"));
        let mut count = 0;
        while let Some(entity) = tests::report_if_err(&cx, Optional::read(&mut reader))? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};")),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, 106);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/reading/optional.sibs"));
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                reader.get_fragment(&entity.token)?.lined,
                "Line: {}",
                count + 1
            );
            // In some cases like with PatternString, semicolon can be skipped, because
            // belongs to parent entity (Optional).
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.action.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.action.token())?.lined
                )),
                "Line: {}",
                count + 1
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.condition.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.condition.token())?.lined
                )),
                "Line: {}",
                count + 1
            );
            count += 1;
        }
        assert_eq!(count, 106);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx: Context = Context::create().unbound()?;
        let samples = include_str!("../tests/error/optional.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
            let opt = Optional::read(&mut reader);
            println!("{opt:?}");
            assert!(opt.is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{chars, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::create().unbound()?;
        let mut reader = cx
            .reader()
            .from_str(include_str!("../tests/processing/optional.sibs"));
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                result.get_as_string().expect("Task returns string value"),
                "true".to_owned()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {

    use crate::{
        elements::{ElTarget, Element, Optional, Task},
        inf::{operator::E, tests::*, Context},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for Optional {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            if deep > MAX_DEEP {
                (
                    Element::arbitrary_with((
                        vec![ElTarget::VariableName, ElTarget::Reference],
                        deep,
                    )),
                    Element::arbitrary_with((
                        vec![
                            ElTarget::Function,
                            ElTarget::Reference,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ],
                        deep,
                    )),
                )
                    .prop_map(|(condition, action)| Optional {
                        condition: Box::new(condition),
                        action: Box::new(action),
                        token: 0,
                    })
                    .boxed()
            } else {
                (
                    Element::arbitrary_with((
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableName,
                            ElTarget::Reference,
                            ElTarget::Block,
                            ElTarget::Comparing,
                        ],
                        deep,
                    )),
                    Element::arbitrary_with((
                        vec![
                            ElTarget::Function,
                            ElTarget::Reference,
                            ElTarget::VariableAssignation,
                            ElTarget::VariableName,
                            ElTarget::Block,
                            ElTarget::Each,
                            ElTarget::First,
                            ElTarget::PatternString,
                            ElTarget::Command,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ],
                        deep,
                    )),
                )
                    .prop_map(|(condition, action)| Optional {
                        condition: Box::new(condition),
                        action: Box::new(action),
                        token: 0,
                    })
                    .boxed()
            }
        }
    }

    fn reading(optional: Optional) -> Result<(), E> {
        get_rt().block_on(async {
            let mut cx = Context::create().unbound()?;
            let origin = format!("test [\n{optional};\n];");
            let mut reader = cx.reader().from_str(&origin);
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            max_shrink_iters: 5000,
            ..ProptestConfig::with_cases(10)
        })]
        #[test]
        fn test_run_task(
            args in any_with::<Optional>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
