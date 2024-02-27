use crate::{
    entry::{Component, Function, PatternString, SimpleString, VariableName},
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
pub enum Element {
    Function(Function),
    PatternString(PatternString),
    VariableName(VariableName),
    SimpleString(SimpleString),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v) => v.to_string(),
                Self::PatternString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::SimpleString(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Element {
    fn token(&self) -> usize {
        match self {
            Self::Function(v) => v.token,
            Self::PatternString(v) => v.token,
            Self::VariableName(v) => v.token,
            Self::SimpleString(v) => v.token,
        }
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Function(v) => v.execute(owner, components, args, cx).await,
                Self::PatternString(v) => v.execute(owner, components, args, cx).await,
                Self::VariableName(v) => v.execute(owner, components, args, cx).await,
                Self::SimpleString(v) => Ok(Some(AnyValue::new(v.to_string()))),
            }
        })
    }
}

impl term::Display for Element {
    fn to_string(&self) -> String {
        format!(
            "[{}]",
            match self {
                Self::Function(v) => v.to_string(),
                Self::PatternString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::SimpleString(v) => v.to_string(),
            }
        )
    }
}

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
            let mut count = 0usize;
            while !inner.rest().trim().is_empty() {
                if let Some((candidate, char)) =
                    inner.until().char(&[&chars::COMMA, &chars::SEMICOLON])
                {
                    if char == chars::SEMICOLON {
                        Err(E::UnexpectedSemicolon.by_reader(reader))?;
                    }
                    if candidate.trim().is_empty() {
                        Err(E::EmptyValue.by_reader(reader))?;
                    }
                    inner.move_to().next();
                    count += 1;
                } else {
                    inner.move_to().end();
                };
                let token = inner.token()?;
                let mut reader = token.bound;
                if let Some(el) = VariableName::read(&mut reader)? {
                    elements.push(Element::VariableName(el));
                    continue;
                }
                if let Some(el) = Function::read(&mut reader)? {
                    elements.push(Element::Function(el));
                    continue;
                }
                if let Some(el) = PatternString::read(&mut reader)? {
                    elements.push(Element::PatternString(el));
                } else if reader.rest().trim().is_empty() {
                    Err(E::EmptyValue.by_reader(&reader))?;
                } else {
                    elements.push(Element::SimpleString(SimpleString {
                        value: reader.rest().trim().to_owned(),
                        token: token.id,
                    }));
                }
            }
            let token = close(reader);
            if count + 1 != elements.len() {
                Err(E::RedundantComma.linked(&token))?;
            }
            Ok(Some(Values { token, elements }))
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
                .join(", ")
        )
    }
}

impl term::Display for Values {
    fn to_string(&self) -> String {
        format!(
            "[{}]",
            self.elements
                .iter()
                .map(term::Display::to_string)
                .collect::<Vec<String>>()
                .join(", ")
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
            let mut values: Vec<String> = vec![];
            for element in self.elements.iter() {
                if let Some(value) = match element {
                    Element::Function(v) => v.execute(owner, components, args, cx).await?,
                    Element::PatternString(v) => v.execute(owner, components, args, cx).await?,
                    Element::VariableName(v) => v.execute(owner, components, args, cx).await?,
                    Element::SimpleString(v) => Some(AnyValue::new(v.to_string())),
                } {
                    if let Some(value) = value.get_as_string() {
                        values.push(value);
                    } else if let Some(value) = value.get_as_strings() {
                        values = [values, value].concat();
                    }
                }
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
        entry::Task,
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
    ];

    #[tokio::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::unbound(include_str!("../tests/processing/values.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            assert!(task.execute(None, &[], &[], &mut cx).await?.is_some());
        }
        for (name, value) in VALUES.iter() {
            assert_eq!(
                cx.get_var(name)
                    .unwrap()
                    .get_as_strings()
                    .unwrap()
                    .join(","),
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
            function::Function,
            pattern_string::PatternString,
            simple_string::SimpleString,
            values::{Element, Values},
            variable_name::VariableName,
        },
        inf::tests::*,
    };
    use proptest::prelude::*;

    impl Arbitrary for Element {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec!["[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|v| Self::SimpleString(SimpleString { value: v, token: 0 }))
                .boxed()];
            if permissions.func {
                allowed.push(
                    Function::arbitrary_with(scope.clone())
                        .prop_map(Self::Function)
                        .boxed(),
                );
            }
            if permissions.value_string {
                allowed.push(
                    PatternString::arbitrary_with(scope.clone())
                        .prop_map(Self::PatternString)
                        .boxed(),
                );
            }
            if permissions.variable_name {
                allowed.push(
                    VariableName::arbitrary()
                        .prop_map(Self::VariableName)
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }

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
