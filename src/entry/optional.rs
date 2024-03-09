use crate::{
    entry::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
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
        if reader.move_to().word(&[words::DO_ON]).is_none() {
            return Err(E::NoOptionalRedirection.by_reader(reader));
        }
        let action = if let Some(el) = Element::include(
            reader,
            &[
                ElTarget::Function,
                ElTarget::Reference,
                ElTarget::VariableAssignation,
                ElTarget::Block,
                ElTarget::Each,
                ElTarget::First,
                ElTarget::PatternString,
                ElTarget::Command,
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
        entry::Optional,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 15);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader =
            Reader::unbound(include_str!("../tests/reading/optional.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = Optional::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                reader.get_fragment(&entity.token)?.lined
            );
            // In some cases like with PatternString, semicolon can be skipped, because
            // belongs to parent entity (Optional).
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.action.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.action.token())?.lined
                )),
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.condition.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.condition.token())?.lined
                )),
            );
            count += 1;
        }
        assert_eq!(count, 15);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/optional.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::Task,
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/optional.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            let result = task
                .execute(None, &[], &[], &mut cx)
                .await?
                .expect("Task returns some value");
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
        entry::{
            optional::{Element, Optional},
            task::Task,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for Optional {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Optional);
            let boxed = (
                Element::arbitrary_with(scope.clone()),
                Element::arbitrary_with(scope.clone()),
            )
                .prop_map(|(condition, action)| Optional {
                    condition: Box::new(condition),
                    action: Box::new(action),
                    token: 0,
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::Optional);
            boxed
        }
    }

    fn reading(optional: Optional) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{optional};\n];");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]
        #[test]
        fn test_run_task(
            args in any_with::<Optional>(Arc::new(RwLock::new(Scope::default())).clone())
        ) {
            prop_assert!(reading(args.clone()).is_ok());
        }
    }
}
