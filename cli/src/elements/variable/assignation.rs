use crate::{
    elements::{Component, ElTarget, Element, VariableName},
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
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
        if let Some(Element::VariableName(variable, _)) =
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
                    ElTarget::Command,
                    ElTarget::VariableName,
                    ElTarget::Integer,
                    ElTarget::Boolean,
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

impl Formation for VariableAssignation {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElTarget::VariableAssignation));
        format!(
            "{}{} = {}",
            cursor.offset_as_string_if(&[ElTarget::Block]),
            self.variable.format(&mut inner),
            self.assignation.format(&mut inner)
        )
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
        cx: Context,
        sc: Scope,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let assignation = &self.assignation;
            let value = assignation
                .execute(owner, components, args, cx, sc.clone())
                .await?
                .ok_or(operator::E::NoValueToAssign(self.variable.name.clone()))?;
            sc.set_var(&self.variable.name, value).await?;
            Ok(Some(AnyValue::new(())))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableAssignation,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{chars, Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(),
            &include_str!("../../tests/reading/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(VariableAssignation::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 113);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(),
            &include_str!("../../tests/reading/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(VariableAssignation::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.lined,
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_carets(&entity.variable.to_string()),
                        trim_carets(&reader.get_fragment(&entity.variable.token)?.content),
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.assignation.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.assignation.token())?.content
                        )),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 113);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/variable_assignation.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(VariableAssignation::read(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Operator, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Reading, Sources},
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
    async fn reading() {
        process_string!(
            &Configuration::logs(),
            &include_str!("../../tests/processing/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::read(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Task>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    assert!(task
                        .execute(None, &[], &[], cx.clone(), sc.clone())
                        .await?
                        .is_some());
                }
                for (name, value) in VALUES.iter() {
                    assert_eq!(
                        sc.get_var(name).await?.unwrap().get_as_string().unwrap(),
                        value.to_string()
                    );
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::{ElTarget, Element, Task, VariableAssignation, VariableName},
        error::LinkedErr,
        inf::{operator::E, tests::*},
        reader::Reading,
    };
    use proptest::prelude::*;

    impl Arbitrary for VariableAssignation {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElTarget::Function,
                            ElTarget::PatternString,
                            ElTarget::Values,
                            ElTarget::Command,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ]
                    } else {
                        vec![
                            ElTarget::Block,
                            ElTarget::First,
                            ElTarget::Function,
                            ElTarget::If,
                            ElTarget::PatternString,
                            ElTarget::Values,
                            ElTarget::Comparing,
                            ElTarget::Command,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ]
                    },
                    deep,
                )),
                VariableName::arbitrary(),
            )
                .prop_map(move |(assignation, variable)| VariableAssignation {
                    assignation: Box::new(assignation),
                    variable,
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(assignation: VariableAssignation) -> Result<(), LinkedErr<E>> {
        get_rt().block_on(async {
            let origin = format!("test [\n{assignation};\n];");
            runner(&origin, |_, mut reader| {
                while let Some(task) = Task::read(&mut reader)? {
                    assert_eq!(format!("{task};"), origin);
                }
                Ok::<(), LinkedErr<E>>(())
            })
            .await?;
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
            args in any_with::<VariableAssignation>(0)
        ) {
            let res = reading(args.clone());
            if res.is_err() {
                println!("{res:?}");
            }
            prop_assert!(res.is_ok());
        }
    }
}
