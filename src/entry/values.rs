use crate::{
    entry::{Component, ElTarget, Element},
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term,
    },
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
                if inner.until().char(&[&chars::SEMICOLON]).is_some() {
                    Err(E::UnrecognizedCode(inner.token()?.content.to_owned())
                        .linked(&inner.token()?.id))?;
                } else {
                    Err(E::UnrecognizedCode(inner.rest().to_owned()).by_reader(&inner))?;
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

impl term::Display for Values {
    fn to_string(&self) -> String {
        format!(
            "({})",
            self.elements
                .iter()
                .map(term::Display::to_string)
                .collect::<Vec<String>>()
                .join("; ")
        )
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
        entry::Values,
        error::LinkedErr,
        inf::{operator::Operator, tests},
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/reading/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(Values::read(&mut reader)?.is_some());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/reading/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            let entity = Values::read(&mut reader)?.unwrap();
            assert_eq!(
                tests::trim_carets(&entity.to_string()),
                reader.get_fragment(&entity.token)?.lined
            );
            for el in entity.elements.iter() {
                assert_eq!(
                    tests::trim_carets(&el.to_string()),
                    tests::trim_carets(&reader.get_fragment(&el.token())?.content)
                );
            }
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../tests/error/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
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
        entry::{Component, Task},
        inf::{
            any::AnyValue,
            context::Context,
            operator::{Operator, E},
        },
        reader::{chars, Reader, Reading},
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
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader = Reader::bound(
            include_str!("../tests/processing/values_components.sibs").to_string(),
            &cx,
        );
        let mut components: Vec<Component> = vec![];
        while let Some(component) = Component::read(&mut reader)? {
            components.push(component);
        }
        let mut reader = Reader::bound(
            include_str!("../tests/processing/values.sibs").to_string(),
            &cx,
        );
        while let Some(task) = Task::read(&mut reader)? {
            let _ = reader.move_to().char(&[&chars::SEMICOLON]);
            assert!(task
                .execute(components.first(), &components, &[], &mut cx)
                .await?
                .is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.get_var(name)
                    .unwrap()
                    .get_as_strings()
                    .unwrap()
                    .join(";"),
                value.to_string()
            );
        }
        for (name, value) in NESTED_VALUES.iter() {
            let stored = cx.get_var(name).unwrap();
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
    use crate::entry::values::{ElTarget, Element, Values};
    use proptest::prelude::*;

    impl Arbitrary for Values {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            let max = 5;
            prop::collection::vec(
                Element::arbitrary_with(vec![
                    ElTarget::Command,
                    ElTarget::Function,
                    ElTarget::If,
                    ElTarget::PatternString,
                    ElTarget::Reference,
                    ElTarget::Values,
                    ElTarget::Comparing,
                    ElTarget::VariableName,
                ]),
                1..max,
            )
            .prop_map(|elements| Values { elements, token: 0 })
            .boxed()
        }
    }
}
