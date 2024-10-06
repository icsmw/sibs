use crate::{
    elements::{Element, ElementRef, TokenGetter},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecuteContext, ExecutePinnedResult, ExpectedResult,
        ExpectedValueType, Formation, FormationCursor, LinkingResult, PrevValueExpectation,
        Processing, TryExecute, TryExpectedValueType, Value, VerificationResult,
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
        let close = reader.open_token(ElementRef::VariableAssignation);
        let global = reader.move_to().word(&[words::GLOBAL_VAR]).is_some();
        if let Some(variable) = Element::include(reader, &[ElementRef::VariableName])? {
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
                    ElementRef::Block,
                    ElementRef::First,
                    ElementRef::Function,
                    ElementRef::If,
                    ElementRef::PatternString,
                    ElementRef::Values,
                    ElementRef::Comparing,
                    ElementRef::Command,
                    ElementRef::VariableName,
                    ElementRef::Integer,
                    ElementRef::Boolean,
                    ElementRef::Reference,
                    ElementRef::Compute,
                    ElementRef::Join,
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
        let mut inner = cursor.reown(Some(ElementRef::VariableAssignation));
        format!(
            "{}{} = {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
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

impl TryExpectedValueType for VariableAssignation {
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
            self.assignation
                .verification(owner, components, prev, cx)
                .await
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
            let Element::VariableName(el, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self));
            };
            self.variable.linking(owner, components, prev, cx).await?;
            self.assignation
                .linking(owner, components, prev, cx)
                .await?;
            cx.variables
                .set(
                    &owner.as_component()?.uuid,
                    el.get_name(),
                    self.assignation
                        .expected(owner, components, prev, cx)
                        .await?,
                )
                .await
                .map_err(|e| LinkedErr::new(e, Some(self.token)))?;
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
        Box::pin(async move { self.assignation.expected(owner, components, prev, cx).await })
    }
}

impl Processing for VariableAssignation {}

impl TryExecute for VariableAssignation {
    fn try_execute<'a>(&'a self, cx: ExecuteContext<'a>) -> ExecutePinnedResult<'a> {
        Box::pin(async move {
            let Element::VariableName(variable, _) = self.variable.as_ref() else {
                return Err(operator::E::NoVariableName.by(self.variable.as_ref()));
            };
            let value = self
                .assignation
                .execute(cx.clone())
                .await?
                .not_empty_or(operator::E::NoValueToAssign(variable.name.clone()))?;
            if self.global {
                cx.sc.set_global_var(&variable.name, value).await?;
            } else {
                cx.sc.set_var(&variable.name, value).await?;
            }
            Ok(Value::empty())
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::{TokenGetter, VariableAssignation},
        error::LinkedErr,
        inf::{tests::*, Configuration},
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
                assert_eq!(count, 113);
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
    use crate::{
        elements::{Element, ElementRef},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, ExecuteContext, Journal, Scope,
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
                    src.report_err_if(Element::include(reader, &[ElementRef::Task]))?
                {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Element>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Element>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    task.execute(ExecuteContext::unbound(cx.clone(), sc.clone()))
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
        elements::{Element, ElementRef, Task, VariableAssignation},
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
                            ElementRef::Function,
                            ElementRef::PatternString,
                            ElementRef::Values,
                            ElementRef::Command,
                            ElementRef::VariableName,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                        ]
                    } else {
                        vec![
                            ElementRef::Block,
                            ElementRef::First,
                            ElementRef::Function,
                            ElementRef::If,
                            ElementRef::PatternString,
                            ElementRef::Values,
                            ElementRef::Comparing,
                            ElementRef::Command,
                            ElementRef::VariableName,
                            ElementRef::Integer,
                            ElementRef::Boolean,
                            ElementRef::Reference,
                            ElementRef::Compute,
                        ]
                    },
                    deep,
                )),
                Element::arbitrary_with((vec![ElementRef::VariableName], deep)),
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
