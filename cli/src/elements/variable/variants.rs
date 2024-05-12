use tokio_util::sync::CancellationToken;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        operator, AnyValue, Context, Formation, FormationCursor, Operator, OperatorPinnedResult,
        Scope,
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
        let mut values: Vec<String> = Vec::new();
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
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let value = if args.len() != 1 {
                Err(operator::E::InvalidNumberOfArgumentsForDeclaration.by(self))?
            } else {
                args[0].to_owned()
            };
            if self.values.contains(&value) {
                Ok(Some(AnyValue::new(value)))
            } else {
                Err(
                    operator::E::NotDeclaredValueAsArgument(value, self.values.join(" | "))
                        .by(self),
                )
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

#[cfg(test)]
mod reading {
    use crate::{
        elements::VariableVariants,
        error::LinkedErr,
        inf::Configuration,
        read_string,
        reader::{Reader, Reading, Sources, E},
    };

    #[tokio::test]
    async fn reading() {
        let samples = include_str!("../../tests/reading/variants.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, src: &mut Sources| {
                    assert!(src.report_err_if(VariableVariants::read(reader)).is_ok());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }

    #[tokio::test]
    async fn error() {
        let samples = include_str!("../../tests/error/variants.sibs");
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            count += read_string!(
                &Configuration::logs(false),
                sample,
                |reader: &mut Reader, _: &mut Sources| {
                    assert!(VariableVariants::read(reader).is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
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
            prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 2..=10)
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
