use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Values {
    pub elements: Vec<Element>,
    pub token: usize,
}

impl Reading<Values> for Values {
    fn read(reader: &mut Reader) -> Result<Option<Values>, LinkedErr<E>> {
        let close = reader.open_token(ElTarget::Values);
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
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
                if inner.move_to().char(&[&chars::SEMICOLON]).is_none()
                    && !inner.rest().trim().is_empty()
                {
                    Err(E::MissedSemicolon.by_reader(&inner))?;
                }
                elements.push(el);
            }
            if !inner.rest().trim().is_empty() {
                if let Some((content, _)) = inner.until().char(&[&chars::SEMICOLON]) {
                    Err(E::UnrecognizedCode(content).by_reader(&inner))?;
                } else {
                    Err(E::UnrecognizedCode(inner.move_to().end()).by_reader(&inner))?;
                }
            }
            Ok(Some(Values {
                token: close(reader),
                elements,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({})",
            self.elements
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("; ")
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
                    .join(";\n"),
                cursor.offset_as_string_if(&[ElTarget::Block, ElTarget::Function])
            )
        } else {
            format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
        }
    }
}

impl Operator for Values {
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
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut values: Vec<AnyValue> = Vec::new();
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
                    .unwrap_or(AnyValue::empty()),
                );
            }
            Ok(Some(AnyValue::new(values)?))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Values,
        error::LinkedErr,
        inf::{operator::Operator, tests::*, Configuration},
        read_string,
        reader::{Reader, Reading, Sources, E},
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
                let entity = src.report_err_if(Values::read(reader))?;
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
                let entity = src.report_err_if(Values::read(reader))?.unwrap();
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
                assert!(Values::read(reader).is_err());
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
            operator::{Operator, E},
            AnyValue, Configuration, Context, Journal, Scope,
        },
        process_string, read_string,
        reader::{chars, Reader, Reading, Sources},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a;b"),
        ("a2", "a;b;c"),
        ("a3", "a;b;c"),
        ("a4", "aa;bb;cc"),
        ("a5", "a:a;b:b"),
    ];
    const NESTED_VALUES: &[(&str, &str)] = &[("a6", "c:a;d:b;d:c")];

    #[tokio::test]
    async fn reading() {
        let components: Vec<Component> = read_string!(
            &Configuration::logs(false),
            &include_str!("../tests/processing/values_components.sibs"),
            |reader: &mut Reader, src: &mut Sources| {
                let mut components: Vec<Component> = Vec::new();
                while let Some(component) = src.report_err_if(Component::read(reader))? {
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
                while let Some(task) = src.report_err_if(Task::read(reader))? {
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
                            .join(";"),
                        value.to_string()
                    );
                }
                for (name, value) in NESTED_VALUES.iter() {
                    let stored = sc.get_var(name).await?.unwrap();
                    let values = stored.get::<Vec<AnyValue>>().unwrap();
                    let mut output: Vec<String> = Vec::new();
                    for value in values.iter() {
                        output = [output, value.as_strings().unwrap()].concat();
                    }
                    assert_eq!(output.join(";"), value.to_string());
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
