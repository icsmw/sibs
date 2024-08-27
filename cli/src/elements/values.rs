use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, GlobalVariablesMap, Scope, TokenGetter, TryExecute, Value, ValueRef,
        ValueTypeResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Values {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl TryDissect<Values> for Values {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Values>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Values);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_none()
        {
            return Ok(None);
        }
        let token = reader.token()?;
        let mut inner = token.bound;
        let mut elements: Vec<Element> = Vec::new();
        if inner.rest().trim().is_empty() {
            return Ok(Some(Values {
                token: close(reader),
                elements,
            }));
        }
        while let Some(el) = Element::include(
            &mut inner,
            &[
                ElTarget::Command,
                ElTarget::Function,
                ElTarget::If,
                ElTarget::PatternString,
                ElTarget::Reference,
                ElTarget::Values,
                ElTarget::Comparing,
                ElTarget::VariableName,
                ElTarget::Integer,
                ElTarget::Boolean,
            ],
        )? {
            if inner.move_to().char(&[&chars::COMMA]).is_none() && !inner.rest().trim().is_empty() {
                Err(E::MissedComma.by_reader(&inner))?;
            }
            elements.push(el);
        }
        if !inner.rest().trim().is_empty() {
            if let Some((content, _)) = inner.until().char(&[&chars::COMMA]) {
                Err(E::UnrecognizedCode(content).by_reader(&inner))?;
            } else {
                Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
            }
        }
        Ok(Some(Values {
            token: close(reader),
            elements,
        }))
    }
}

impl Dissect<Values, Values> for Values {}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Formation for Values {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.to_string().len() > cursor.max_len() && self.elements.len() > cursor.max_items() {
            format!(
                "{}(\n{}\n{})",
                cursor.offset_as_string_if(&[ElTarget::Block]),
                self.elements
                    .iter()
                    .map(|v| format!(
                        "{}{}",
                        cursor.right().offset_as_string(),
                        v.format(&mut cursor.reown(Some(ElTarget::Values)).right())
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                cursor.offset_as_string_if(&[ElTarget::Block, ElTarget::Function])
            )
        } else {
            format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
        }
    }
}

impl TokenGetter for Values {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for Values {
    fn varification<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        Ok(())
    }
    fn linking<'a>(
        &'a self,
        _variables: &mut GlobalVariablesMap,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> Result<(), LinkedErr<operator::E>> {
        Ok(())
    }
    fn expected<'a>(
        &'a self,
        _owner: &'a Component,
        _components: &'a [Component],
    ) -> ValueTypeResult {
        Ok(ValueRef::Vec)
    }
}

impl TryExecute for Values {
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
            let mut values: Vec<Value> = Vec::new();
            for el in self.elements.iter() {
                values.push(
                    el.execute(
                        owner,
                        components,
                        args,
                        cx.clone(),
                        sc.clone(),
                        token.clone(),
                    )
                    .await?
                    .unwrap_or(Value::empty()),
                );
            }
            Ok(Some(Value::Vec(values)))
        })
    }
}

impl Execute for Values {}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Values,
        error::LinkedErr,
        inf::{operator::TokenGetter, tests::*, Configuration},
        read_string,
        reader::{Dissect, Reader, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?;
                assert!(entity.is_some(), "Line: {}", count + 1);
                let entity = entity.unwrap();
                assert_eq!(
                    trim_carets(reader.recent()),
                    trim_carets(&format!("{entity}")),
                    "Line: {}",
                    count + 1
                );
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn tokens() {
        let samples = include_str!("../tests/reading/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, src: &mut Sources| {
                let mut count = 0;
                let entity = src.report_err_if(Values::dissect(reader))?.unwrap();
                assert_eq!(
                    trim_carets(&entity.to_string()),
                    reader.get_fragment(&entity.token)?.lined,
                    "Line: {}",
                    count + 1
                );
                for el in entity.elements.iter() {
                    assert_eq!(
                        trim_carets(&el.to_string()),
                        trim_carets(&reader.get_fragment(&el.token())?.content),
                        "Line: {}",
                        count + 1
                    );
                }
                count += 1;
                Ok::<usize, LinkedErr<E>>(count)
            });
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../tests/error/values.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        let cfg = Configuration::logs(false);
        for sample in samples.iter() {
            count += read_string!(&cfg, sample, |reader: &mut Reader, _: &mut Sources| {
                let entity = Values::dissect(reader);
                assert!(entity.is_err());
                Ok::<usize, LinkedErr<E>>(1)
            });
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod processing {
    use tokio_util::sync::CancellationToken;

    use crate::{
        elements::{Component, Task},
        error::LinkedErr,
        inf::{
            operator::{Execute, E},
            Configuration, Context, Journal, Scope, Value,
        },
        process_string, read_string,
        reader::{chars, Dissect, Reader, Sources},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a,b"),
        ("a2", "a,b,c"),
        ("a3", "a,b,c"),
        ("a4", "aa,bb,cc"),
        ("a5", "a:a,b:b"),
    ];
    const NESTED_VALUES: &[(&str, &str)] = &[("a6", "c:a,d:b,d:c")];

    #[tokio::test]
    async fn reading() {
        let components: Vec<Component> = read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/values_components.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Component> = Vec::new();
                while let Some(component) = src.report_err_if(Component::dissect(reader))? {
                    components.push(component);
                }
                Ok::<Vec<Component>, LinkedErr<E>>(components)
            }
        );
        process_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/values.sibs"),
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
                    assert!(task
                        .execute(
                            components.first(),
                            &components,
                            &[],
                            cx.clone(),
                            sc.clone(),
                            CancellationToken::new()
                        )
                        .await?
                        .is_some());
                }
                for (name, value) in VALUES.iter() {
                    assert_eq!(
                        sc.get_var(name)
                            .await?
                            .unwrap()
                            .as_strings()
                            .unwrap()
                            .join(","),
                        value.to_string()
                    );
                }
                for (name, value) in NESTED_VALUES.iter() {
                    let stored = sc.get_var(name).await?.unwrap();
                    let values = stored.get::<Vec<Value>>().unwrap();
                    let mut output: Vec<String> = Vec::new();
                    for value in values.iter() {
                        output = [output, value.as_strings().unwrap()].concat();
                    }
                    assert_eq!(output.join(","), value.to_string());
                }
                Ok::<(), LinkedErr<E>>(())
            }
        );
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        elements::values::{ElTarget, Element, Values},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Values {
        type Parameters = usize;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(deep: Self::Parameters) -> Self::Strategy {
            let max = 5;
            prop::collection::vec(
                Element::arbitrary_with((
                    if deep > MAX_DEEP {
                        vec![ElTarget::VariableName, ElTarget::Integer, ElTarget::Boolean]
                    } else {
                        vec![
                            ElTarget::Command,
                            ElTarget::Function,
                            ElTarget::If,
                            ElTarget::PatternString,
                            ElTarget::Reference,
                            ElTarget::Values,
                            ElTarget::Comparing,
                            ElTarget::VariableName,
                            ElTarget::Integer,
                            ElTarget::Boolean,
                        ]
                    },
                    deep,
                )),
                1..max,
            )
            .prop_map(|elements| Values { elements, token: 0 })
            .boxed()
        }
    }
}
