use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{Operator, OperatorPinnedResult},
        term::{self, Term},
    },
    reader::{
        chars,
        entry::{Component, Function, Reader, Reading, ValueString, VariableName},
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Element {
    Function(Function),
    ValueString(ValueString),
    VariableName(VariableName),
    String(String),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Function(v) => v.to_string(),
                Self::ValueString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::String(v) => v.to_string(),
            }
        )
    }
}

impl Operator for Element {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            match self {
                Self::Function(v) => v.process(owner, components, args, cx).await,
                Self::ValueString(v) => v.process(owner, components, args, cx).await,
                Self::VariableName(v) => v.process(owner, components, args, cx).await,
                Self::String(v) => Ok(Some(AnyValue::new(v.to_owned()))),
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
                Self::ValueString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::String(v) => v.to_string(),
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
    fn read(reader: &mut Reader) -> Result<Option<Values>, E> {
        if reader
            .group()
            .between(&chars::OPEN_BRACKET, &chars::CLOSE_BRACKET)
            .is_some()
        {
            let token = reader.token()?;
            let mut inner = token.bound;
            let mut elements: Vec<Element> = vec![];
            if inner.rest().trim().is_empty() {
                Err(E::EmptyValue)?;
            }
            while !inner.rest().trim().is_empty() {
                if let Some((candidate, char)) =
                    inner.until().char(&[&chars::COMMA, &chars::SEMICOLON])
                {
                    if char == chars::SEMICOLON {
                        Err(E::UnexpectedSemicolon)?;
                    }
                    if candidate.trim().is_empty() {
                        Err(E::EmptyValue)?;
                    }
                    inner.move_to().next();
                } else {
                    inner.move_to().end();
                };
                let mut reader = inner.token()?.bound;
                if let Some(el) = VariableName::read(&mut reader)? {
                    elements.push(Element::VariableName(el));
                    continue;
                }
                if let Some(el) = Function::read(&mut reader)? {
                    elements.push(Element::Function(el));
                    continue;
                }
                if let Some(el) = ValueString::read(&mut reader)? {
                    elements.push(Element::ValueString(el));
                } else if reader.rest().trim().is_empty() {
                    Err(E::EmptyValue)?;
                } else {
                    elements.push(Element::String(reader.rest().trim().to_owned()));
                }
            }
            Ok(Some(Values {
                token: token.id,
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
    fn process<'a>(
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
                    Element::Function(v) => v.process(owner, components, args, cx).await?,
                    Element::ValueString(v) => v.process(owner, components, args, cx).await?,
                    Element::VariableName(v) => v.process(owner, components, args, cx).await?,
                    Element::String(v) => Some(AnyValue::new(v.to_owned())),
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
    use crate::reader::{
        entry::{Reading, Values},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let samples = include_str!("../../tests/reading/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Values::read(&mut reader).is_ok());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/values.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
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
        inf::{
            context::Context,
            operator::{Operator, E},
        },
        reader::{
            entry::{Reading, Task},
            Reader,
        },
    };

    const VALUES: &[(&str, &str)] = &[
        ("a0", "a"),
        ("a1", "a,b"),
        ("a2", "a,b,c"),
        ("a3", "a,b,c"),
        ("a4", "aa,bb,cc"),
    ];

    #[async_std::test]
    async fn reading() -> Result<(), E> {
        let mut cx = Context::unbound()?;
        let mut reader =
            Reader::new(include_str!("../../tests/processing/values.sibs").to_string());
        while let Some(task) = Task::read(&mut reader)? {
            assert!(task.process(None, &[], &[], &mut cx).await?.is_some());
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

// #[cfg(test)]
// mod proptest {
//     use crate::{inf::tests::*, reader::entry::values::Values};
//     use proptest::prelude::*;

//     impl Arbitrary for Values {
//         type Parameters = SharedScope;
//         type Strategy = BoxedStrategy<Self>;

//         fn arbitrary_with(_scope: Self::Parameters) -> Self::Strategy {
//             prop::collection::vec("[a-zA-Z_][a-zA-Z0-9_]*".prop_map(String::from), 0..=10)
//                 .prop_map(|values| Values { values, token: 0 })
//                 .boxed()
//         }
//     }
// }
