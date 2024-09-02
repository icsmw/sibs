use tokio_util::sync::CancellationToken;

use crate::{
    elements::{ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedResult, ExpectedValueType,
        Formation, FormationCursor, GlobalVariablesMap, LinkingResult, Scope, TokenGetter,
        TryExecute, Value, VerificationResult,
    },
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableAssignation {
    pub variable: Box<Element>,
    pub global: bool,
    pub assignation: Box<Element>,
    pub token: usize,
}

impl TryDissect<VariableAssignation> for VariableAssignation {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableAssignation>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::VariableAssignation);
        let global = reader.move_to().word(&[words::GLOBAL_VAR]).is_some();
        if let Some(variable) = Element::include(reader, &[ElTarget::VariableName])? {
            let rest = reader.rest().trim();
            if rest.starts_with(words::DO_ON)
                || rest.starts_with(words::CMP_TRUE)
                || !rest.starts_with(chars::EQUAL)
            {
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
                    ElTarget::Reference,
                ],
            )?
            .ok_or(E::FailToParseRightSideOfAssignation.by_reader(reader))?;
            Ok(Some(VariableAssignation {
                variable: Box::new(variable),
                global,
                assignation: Box::new(assignation),
                token: close(reader),
            }))
        } else if global {
            Err(E::InvalidUsageGlobalKeyword.by_reader(reader))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<VariableAssignation, VariableAssignation> for VariableAssignation {}

impl fmt::Display for VariableAssignation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} = {}",
            if self.global {
                format!("{} ", words::GLOBAL_VAR)
            } else {
                String::new()
            },
            self.variable,
            self.assignation
        )
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

impl TokenGetter for VariableAssignation {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for VariableAssignation {
    fn varification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _cx: &'a Context,
    ) -> VerificationResult {
        Box::pin(async move { Ok(()) })
    }

    fn linking<'a>(
        &'a self,
        variables: &'a mut GlobalVariablesMap,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> LinkingResult {
        Box::pin(async move {
            let Element::VariableName(el, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self));
            };
            variables
                .set(
                    &owner.as_component()?.uuid,
                    el.get_name(),
                    self.assignation.expected(owner, components, cx).await?,
                )
                .map_err(|e| LinkedErr::new(e, Some(self.token)))?;
            Ok(())
        })
    }
    fn expected<'a>(
        &'a self,
        owner: &'a Element,
        components: &'a [Element],
        cx: &'a Context,
    ) -> ExpectedResult {
        Box::pin(async move { self.assignation.expected(owner, components, cx).await })
    }
}

impl TryExecute for VariableAssignation {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Element>,
        components: &'a [Element],
        args: &'a [Value],
        prev: &'a Option<Value>,
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let Element::VariableName(variable, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self.variable.as_ref()));
            };
            let value = self
                .assignation
                .execute(owner, components, args, prev, cx, sc.clone(), token)
                .await?
                .not_empty_or(operator::E::NoValueToAssign(variable.name.clone()))?;
            if self.global {
                sc.set_global_var(&variable.name, value).await?;
            } else {
                sc.set_var(&variable.name, value).await?;
            }
            Ok(Value::empty())
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableAssignation,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(VariableAssignation::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 112);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/reading/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(VariableAssignation::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.lined,
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_carets(&entity.variable.to_string()),
                        trim_carets(&reader.get_fragment(&entity.variable.token())?.content),
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
                assert_eq!(count, 112);
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
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(VariableAssignation::dissect(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{ElTarget, Element},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Reader, Sources},
    };

    const VALUES: &[(&str, &str, bool)] = &[
        ("a", "a", false),
        ("b", "b", false),
        ("c", "abc", false),
        ("d", "ababc", false),
        ("e", "ababc", false),
        ("f", "\\{$a\\}\\{$b\\}\\{$c\\}", false),
        ("g", "\\{$a\\}\\{$b\\}\\{$c\\}", true),
    ];

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../../tests/processing/variable_assignation.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Element> = Vec::new();
                while let Some(task) =
                    src.report_err_if(Element::include(reader, &[ElTarget::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    task.execute(
                        None,
                        &[],
                        &[],
                        &None,
                        cx.clone(),
                        sc.clone(),
                        CancellationToken::new(),
                    )
                    .await?;
                }
                for (name, value, global) in VALUES.iter() {
                    assert_eq!(
                        if *global {
                            sc.get_global_var(name).await?
                        } else {
                            sc.get_var(name).await?
                        }
                        .unwrap()
                        .as_string()
                        .unwrap(),
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
        elements::{ElTarget, Element, Task, VariableAssignation},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
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
                            ElTarget::Reference,
                        ]
                    },
                    deep,
                )),
                Element::arbitrary_with((vec![ElTarget::VariableName], deep)),
                prop_oneof![Just(true), Just(false),].boxed(),
            )
                .prop_map(move |(assignation, variable, global)| VariableAssignation {
                    assignation: Box::new(assignation),
                    global,
                    variable: Box::new(variable),
                    token: 0,
                })
                .boxed()
        }
    }

    fn reading(assignation: VariableAssignation) {
        get_rt().block_on(async {
            let origin = format!("@test {{\n{assignation};\n}};");
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
            args in any_with::<VariableAssignation>(0)
        ) {
            reading(args.clone());
        }
    }
}
