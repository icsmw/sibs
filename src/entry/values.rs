use crate::{
    entry::{Component, Element},
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
            println!(">>>>>>>>>>>>>>>>>>>>VALUES TO READ:__{}__", inner.rest());
            while let Some(el) = Element::read(&mut inner)? {
                elements.push(el);
                if inner.move_to().char(&[&chars::SEMICOLON]).is_none()
                    && !inner.rest().trim().is_empty()
                {
                    println!(">>>>>>>>>>>>>>>>>>>>ERR:__{}__", inner.rest());
                    Err(E::MissedSemicolon.by_reader(&inner))?;
                }
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
                .map(|el| el.to_string())
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
            let mut values: Vec<Option<AnyValue>> = vec![];
            for el in self.elements.iter() {
                values.push(el.execute(owner, components, args, cx).await?);
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
            context::Context,
            operator::{Operator, E},
        },
        reader::{Reader, Reading},
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a,b"),
        ("a2", "a,b,c"),
        ("a3", "a,b,c"),
        ("a4", "aa,bb,cc"),
        ("a5", "a:a,b:b"),
        ("a6", "c:a,d:b,d:c"),
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/values_components.sibs").to_string());
        let mut components: Vec<Component> = vec![];
        while let Some(component) = Component::read(&mut reader)? {
            components.push(component);
        }

        let mut reader =
            Reader::unbound(include_str!("../tests/processing/values.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
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
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        entry::values::{Element, Values},
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Values {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            scope.write().unwrap().include(Entity::Values);
            let max = 5;
            let boxed = prop::collection::vec(Element::arbitrary_with(scope.clone()), 1..max)
                .prop_map(|elements| Values { elements, token: 0 })
                .boxed();
            scope.write().unwrap().exclude(Entity::Values);
            boxed
        }
    }
}
