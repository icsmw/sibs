use tokio_util::sync::CancellationToken;

use crate::{
    elements::Component,
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, ExpectedValueType, Formation,
        FormationCursor, Scope, TokenGetter, TryExecute, Value, ValueRef, ValueTypeResult,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableVariants {
    pub values: Vec<Value>,
    pub token: usize,
}

impl TryDissect<VariableVariants> for VariableVariants {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let content = reader
            .until()
            .char(&[&chars::SEMICOLON, &chars::COMMA])
            .map(|(content, _)| content)
            .unwrap_or_else(|| reader.move_to().end());
        Ok(Some(VariableVariants::new(content, reader.token()?.id)?))
    }
}

impl Dissect<VariableVariants, VariableVariants> for VariableVariants {}

impl VariableVariants {
    pub fn new(input: String, token: usize) -> Result<Self, LinkedErr<E>> {
        let mut values: Vec<Value> = Vec::new();
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
            if let Ok(num) = value.parse::<isize>() {
                values.push(Value::isize(num))
            } else {
                values.push(Value::String(value.to_string()));
            }
        }
        if values.is_empty() {
            Err(E::NoVariableValues.linked(&token))?;
        }
        Ok(VariableVariants { values, token })
    }
}

impl TokenGetter for VariableVariants {
    fn token(&self) -> usize {
        self.token
    }
}

impl ExpectedValueType for VariableVariants {
    fn expected<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
    ) -> ValueTypeResult {
        // TODO: needs implementation
        Ok(ValueRef::String)
    }
}

impl TryExecute for VariableVariants {
    fn try_execute<'a>(
        &'a self,
        _owner: Option<&'a Component>,
        _components: &'a [Component],
        args: &'a [Value],
        _cx: Context,
        _sc: Scope,
        _token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move {
            let value = if args.len() != 1 {
                Err(operator::E::InvalidNumberOfArgumentsForDeclaration.by(self))?
            } else {
                args[0].to_owned()
            };
            if self.values.contains(&value) {
                Ok(Some(value))
            } else {
                Err(operator::E::NotDeclaredValueAsArgument(
                    value.to_string(),
                    self.values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" | "),
                )
                .by(self))
            }
        })
    }
}

impl Execute for VariableVariants {}

impl fmt::Display for VariableVariants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.values
                .iter()
                .map(|v| v.as_string().expect("Value variant can be only String"))
                .collect::<Vec<String>>()
                .join(" | ")
        )
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
        reader::{Dissect, Reader, Sources, E},
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
                    assert!(src.report_err_if(VariableVariants::dissect(reader)).is_ok());
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
                    let result = VariableVariants::dissect(reader);
                    assert!(result.is_err());
                    Ok::<usize, LinkedErr<E>>(1)
                }
            );
        }
        assert_eq!(count, samples.len());
    }
}

#[cfg(test)]
mod proptest {
    use crate::{elements::VariableVariants, inf::Value};
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
                            Value::String(if v.is_empty() {
                                "min".to_owned()
                            } else {
                                v.to_owned()
                            })
                        })
                        .collect::<Vec<Value>>(),
                    token: 0,
                })
                .boxed()
        }
    }
}
