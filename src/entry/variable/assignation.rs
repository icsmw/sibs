use crate::{
    entry::{Component, ElTarget, Element, VariableName},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableAssignation {
    pub variable: VariableName,
    pub assignation: Box<Element>,
    pub token: usize,
}

impl Reading<VariableAssignation> for VariableAssignation {
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, LinkedErr<E>> {
        let restore = reader.pin();
        let close = reader.open_token();
        if let Some(Element::VariableName(variable)) =
            Element::include(reader, &[ElTarget::VariableName])?
        {
            let rest = reader.rest().trim();
            if rest.starts_with(words::DO_ON)
                || rest.starts_with(words::CMP_TRUE)
                || !rest.starts_with(chars::EQUAL)
            {
                restore(reader);
                return Ok(None);
            }
            let _ = reader.move_to().char(&[&chars::EQUAL]);
            let assignation = Element::include(
                reader,
                &[
                    ElTarget::Block,
                    ElTarget::First,
                    ElTarget::Function,
                    ElTarget::If,
                    ElTarget::PatternString,
                    ElTarget::Values,
                    ElTarget::Comparing,
                    ElTarget::VariableName,
                ],
            )?
            .ok_or(E::FailToParseRightSideOfAssignation.by_reader(reader))?;
            Ok(Some(VariableAssignation {
                variable,
                assignation: Box::new(assignation),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for VariableAssignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.variable, self.assignation)
    }
}

impl Operator for VariableAssignation {
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
            let assignation = &self.assignation;
            let value = assignation
                .execute(owner, components, args, cx)
                .await?
                .ok_or(operator::E::NoValueToAssign(self.variable.name.clone()))?;
            cx.set_var(self.variable.name.clone(), value);
            Ok(Some(AnyValue::new(())))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        entry::VariableAssignation,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{chars, Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(
            include_str!("../../tests/reading/variable_assignation.sibs").to_string(),
        );
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(reader.recent()),
                tests::trim_carets(&format!("{entity};"))
            );
            count += 1;
        }
        assert_eq!(count, 13);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let mut reader = Reader::unbound(
            include_str!("../../tests/reading/variable_assignation.sibs").to_string(),
        );
        let mut count = 0;
        while let Some(entity) = VariableAssignation::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert_eq!(
                tests::trim_carets(&format!("{entity}")),
                reader.get_fragment(&entity.token)?.lined
            );
            assert_eq!(
                tests::trim_carets(&entity.variable.to_string()),
                tests::trim_carets(&reader.get_fragment(&entity.variable.token)?.content)
            );
            assert_eq!(
                tests::trim_semicolon(&tests::trim_carets(&entity.assignation.to_string())),
                tests::trim_semicolon(&tests::trim_carets(
                    &reader.get_fragment(&entity.assignation.token())?.content
                ))
            );
            count += 1;
        }
        assert_eq!(count, 13);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
    #[test]
    fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/error/variable_assignation.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(VariableAssignation::read(&mut reader).is_err());
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

    const VALUES: &[(&str, &str)] = &[
        ("a", "a"),
        ("b", "b"),
        ("c", "abc"),
        ("d", "ababc"),
        ("e", "ababc"),
        ("f", "\\{$a\\}\\{$b\\}\\{$c\\}"),
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../../tests/processing/variable_assignation.sibs").to_string(),
            &cx,
        );
        while let Some(task) = Task::read(&mut reader)? {
            assert!(task.execute(None, &[], &[], &mut cx).await?.is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.get_var(name).unwrap().get_as_string().unwrap(),
                value.to_string()
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::{
            element::Element, task::Task, variable::VariableAssignation, variable::VariableName,
        },
        inf::{operator::E, tests::*},
        reader::{Reader, Reading},
    };
    use proptest::prelude::*;
    use std::sync::{Arc, RwLock};

    impl Arbitrary for VariableAssignation {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::VariableAssignation);
            let inner = scope.clone();
            let boxed = (
                Element::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
            )
                .prop_map(move |(assignation, variable)| {
                    inner
                        .write()
                        .unwrap()
                        .add_assignation(variable.name.clone());
                    VariableAssignation {
                        assignation: Box::new(assignation),
                        variable,
                        token: 0,
                    }
                })
                .boxed();
            scope.write().unwrap().exclude(Entity::VariableAssignation);
            boxed
        }
    }

    fn reading(assignation: VariableAssignation) -> Result<(), E> {
        get_rt().block_on(async {
            let origin = format!("test [\n{assignation};\n];");
            let mut reader = Reader::unbound(origin.clone());
            while let Some(task) = Task::read(&mut reader)? {
                assert_eq!(format!("{task};"), origin);
            }
            Ok(())
        })
    }

    // proptest! {
    //     #![proptest_config(ProptestConfig::with_cases(10))]
    //     #[test]
    //     fn test_run_task(
    //         args in any_with::<VariableAssignation>(Arc::new(RwLock::new(Scope::default())).clone())
    //     ) {
    //         prop_assert!(reading(args.clone()).is_ok());
    //     }
    // }
}
