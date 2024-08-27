use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, Scope, TokenGetter, TryExecute, Value, ValueRef,
        ValueTypeResult,
    },
    reader::{words, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Optional {
    pub condition: Box<Element>,
    pub action: Box<Element>,
    pub token: usize,
}

impl TryDissect<Optional> for Optional {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::DO_ON) {
            return Ok(None);
        }
        let close = reader.open_token(ElTarget::Optional);
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

impl Dissect<Optional, Optional> for Optional {}

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

impl TokenGetter for Optional {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Optional {
    fn varification<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        self.condition.varification(owner, components)?;
        self.action.varification(owner, components)
    }
    fn linking<'a>(
        &'a self,
        variables: &mut GlobalVariablesMap,
        owner: &'a Component,
        components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        self.condition.linking(variables, owner, components)?;
        self.action.linking(variables, owner, components)
    }
    fn expected<'a>(
        &'a self,
        owner: &'a Component,
        components: &'a [Component],
    ) -> ValueTypeResult {
        self.action.expected(owner, components)
    }
}

impl TryExecute for Optional {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [Value],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let condition = *self
                .condition
                .execute(
                    owner,
                    components,
                    args,
                    cx.clone(),
                    sc.clone(),
                    token.clone(),
                )
                .await?
                .ok_or(operator::E::FailToExtractConditionValue)?
                .get::<bool>()
                .ok_or(operator::E::FailToExtractConditionValue)?;
            if !condition {
                Ok(None)
            } else {
                self.action
                    .execute(owner, components, args, cx, sc, token)
                    .await
            }
        })
    }
}

impl Execute for Optional {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Optional,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{chars, Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/optional.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Optional::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(reader.recent()),
                        trim_carets(&format!("{entity};")),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 106);
                assert!(reader.rest().trim().is_empty());
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn tokens() {
        read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/reading/optional.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                while let Some(entity) = src.report_err_if(Optional::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    assert_eq!(
                        trim_carets(&format!("{entity}")),
                        reader.get_fragment(&entity.token)?.lined,
                        "Line: {}",
                        count + 1
                    );
                    // In some cases like with PatternString, semicolon can be skipped, because
                    // belongs to parent entity (Optional).
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.action.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.action.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    assert_eq!(
                        trim_semicolon(&trim_carets(&entity.condition.to_string())),
                        trim_semicolon(&trim_carets(
                            &reader.get_fragment(&entity.condition.token())?.lined
                        )),
                        "Line: {}",
                        count + 1
                    );
                    count += 1;
                }
                assert_eq!(count, 106);
                assert!(reader.rest().trim().is_empty());

                Ok::<(), LinkedErr<E>>(())
            }
        );
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/optional.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    let opt = Optional::dissect(reader);
                    println!("{opt:?}");
                    println!("{}", reader.rest());
                    assert!(opt.is_err());
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
        elements::Task,
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope,
        },
        process_string,
        reader::{chars, Dissect, Reader, Sources},
    };

    #[tokio::test]
    async fn reading() {
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/optional.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut tasks: Vec<Task> = Vec::new();
                while let Some(task) = src.report_err_if(Task::dissect(reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
            |tasks: Vec<Task>, cx: Context, sc: Scope, _: Journal| async move {
                for task in tasks.iter() {
                    let result = task
                        .execute(
                            None,
                            &[],
                            &[],
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new(),
                        )
                        .await?
                        .expect("Task returns some value");
                    assert_eq!(
                        result.as_string().expect("Task returns string value"),
                        "true".to_owned()
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
        elements::{ElTarget, Element, Optional, Task},
        error::LinkedErr,
        inf::{operator::E, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources},
    };
    use proptest::prelude::*;

    impl Arbitrary for Optional {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            (
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::VariableName, ElTarget::Reference]
                    } else {
                        vec![
                            ElTarget::Function,
                            ElTarget::VariableName,
                            ElTarget::Reference,
                            ElTarget::Block,
                            ElTarget::Comparing,
                        ]
                    },
                    deep,
                )),
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![
                            ElTarget::Function,
                            ElTarget::Reference,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ]
                    } else {
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
                        ]
                    },
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

    fn reading(optional: Optional) {
        get_rt().block_on(async {
            let origin = format!("test {{\n{optional};\n}};");
            read_string!(
                &Configuration::logs(false),
                &origin,
                |reader: &mut Reader, src: &mut Sources| {
                    while let Some(task) = src.report_err_if(Task::dissect(reader))? {
                        assert_eq!(format!("{task};"), origin);
                    }
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
            args in any_with::<Optional>(0)
        ) {
            reading(args.clone());
        }
    }
}
