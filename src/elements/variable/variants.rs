use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        operator, term, AnyValue, Context, Formation, FormationCursor, Operator,
        OperatorPinnedResult,
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableVariants {
    pub values: Vec<String>,
    pub token: usize,
}

impl Reading<VariableVariants> for VariableVariants {
    fn read(reader: &mut Reader) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let content = reader
            .until()
            .char(&[&chars::SEMICOLON])
            .map(|(content, _)| content)
            .unwrap_or_else(|| reader.move_to().end());
        Ok(Some(VariableVariants::new(content, reader.token()?.id)?))
    }
}

impl VariableVariants {
    pub fn new(input: String, token: usize) -> Result<Self, LinkedErr<E>> {
        let mut values: Vec<String> = vec![];
        for value in input.split('|') {
            let value = value.trim();
            if !value.is_ascii() {
                Err(E::NotAsciiValue(value.to_string()).linked(&token))?;
            }
            if chars::has_reserved(value) {
                Err(E::UsingReservedChars.linked(&token))?
            }
            if value.is_empty() {
                Err(E::EmptyValue.linked(&token))?;
            }
            values.push(value.to_string());
        }
        if values.is_empty() {
            Err(E::NoVariableValues.linked(&token))?;
        }
        Ok(VariableVariants { values, token })
    }

    pub fn parse(&self, value: String) -> Result<Option<String>, operator::E> {
        if self.values.contains(&value) {
            Ok(Some(value))
        } else {
            Err(operator::E::NotDeclaredValueAsArgument(
                value,
                self.values.join(" | "),
            ))
        }
    }
}

impl Operator for VariableVariants {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        args: &'a [String],
        _cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let value = if args.len() != 1 {
                Err(operator::E::InvalidNumberOfArgumentsForDeclaration)?
            } else {
                args[0].to_owned()
            };
            if self.values.contains(&value) {
                Ok(Some(AnyValue::new(value)))
            } else {
                Err(operator::E::NotDeclaredValueAsArgument(
                    value,
                    self.values.join(" | "),
                ))
            }
        })
    }
}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values.join(" | "))
    }
}

impl Formation for VariableVariants {
    fn elements_count(&self) -> usize {
        self.values.len()
    }
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}

impl term::Display for VariableVariants {
    fn to_string(&self) -> String {
        format!("[{}]", self.values.join(" | "))
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableVariants,
        error::LinkedErr,
        inf::{context::Context, tests::*},
        reader::{Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let mut cx: Context = Context::create().unbound()?;
        let samples = include_str!("../../tests/reading/variants.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
            assert!(report_if_err(&cx, VariableVariants::read(&mut reader)).is_ok());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[tokio::test]
    async fn error() -> Result<(), E> {
        let mut cx: Context = Context::create().unbound()?;
        let samples = include_str!("../../tests/error/variants.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = cx.reader().from_str(sample);
            assert!(VariableVariants::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::elements::VariableVariants;
    use proptest::prelude::*;

    impl Arbitrary for VariableVariants {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 0..=10)
                .prop_map(|values| VariableVariants {
                    values: values
                        .iter()
                        .map(|v| {
                            if v.is_empty() {
                                "min".to_owned()
                            } else {
                                v.to_owned()
                            }
                        })
                        .collect::<Vec<String>>(),
                    token: 0,
                })
                .boxed()
        }
    }
}
