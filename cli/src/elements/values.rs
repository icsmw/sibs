use crate::{
    elements::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{term, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult},
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
        let close = reader.open_token();
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let token = reader.token()?;
            let mut inner = token.bound;
            let mut elements: Vec<Element> = vec![];
            if inner.rest().trim().is_empty() {
                Err(E::EmptyValue.linked(&token.id))?;
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
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let mut values: Vec<AnyValue> = vec![];
            for el in self.elements.iter() {
                values.push(
                    el.execute(owner, components, args, cx)
                        .await?
                        .unwrap_or(AnyValue::new(())),
                );
            }
            Ok(Some(AnyValue::new(values)))
        })
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Values,
        error::LinkedErr,
        inf::{operator::Operator, tests::*},
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/reading/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += runner(sample, |mut src, mut reader| {
                let mut count = 0;
                let entity = src.report_err_if(Values::read(&mut reader))?;
                assert!(entity.is_some(), "Line: {}", count + 1);
                let entity = entity.unwrap();
                assert_eq!(
                    trim_carets(reader.recent()),
                    trim_carets(&format!("{entity}")),
                    "Line: {}",
                    count + 1
                );
                count += 1;
                Ok(count)
            })?;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/reading/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += runner(sample, |_, mut reader| {
                let mut count = 0;
                let entity = Values::read(&mut reader)?.unwrap();
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
                Ok(count)
            })?;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/error/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let (_, mut reader) = get_reader_for_str(sample);
            assert!(Values::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod processing {
    use crate::{
        elements::{Component, Task},
        error::LinkedErr,
        inf::{
            any::AnyValue,
            context::Context,
            operator::{Operator, E},
            tests::*,
        },
        reader::{chars, Reading},
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
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx = Context::create().unbound()?;
        let components: Vec<Component> = runner(
            &include_str!("../tests/processing/values_components.sibs"),
            |_, mut reader| {
                let mut components: Vec<Component> = vec![];
                while let Some(component) = Component::read(&mut reader)? {
                    components.push(component);
                }
                Ok::<Vec<Component>, LinkedErr<E>>(components)
            },
        )?;
        let tasks: Vec<Task> = runner(
            &include_str!("../tests/processing/values.sibs"),
            |_, mut reader| {
                let mut tasks: Vec<Task> = vec![];
                while let Some(task) = Task::read(&mut reader)? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    tasks.push(task);
                }
                Ok::<Vec<Task>, LinkedErr<E>>(tasks)
            },
        )?;
        for task in tasks.iter() {
            assert!(task
                .execute(components.first(), &components, &[], &mut cx)
                .await?
                .is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.vars()
                    .get(name)
                    .unwrap()
                    .get_as_strings()
                    .unwrap()
                    .join(";"),
                value.to_string()
            );
        }
        for (name, value) in NESTED_VALUES.iter() {
            let binding = cx.vars();
            let stored = binding.get(name).unwrap();
            let values = stored.get_as::<Vec<AnyValue>>().unwrap();
            let mut output: Vec<String> = vec![];
            for value in values.iter() {
                output = [output, value.get_as_strings().unwrap()].concat();
            }
            assert_eq!(output.join(";"), value.to_string());
        }
        Ok(())
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
