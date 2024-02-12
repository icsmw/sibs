use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Cmp, Component, Reader, Reading, VariableName},
        words, E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableComparing {
    pub name: VariableName,
    pub cmp: Cmp,
    pub value: String,
    pub token: usize,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, E> {
        reader.state().set();
        let close = reader.open_token();
        if let Some(name) = VariableName::read(reader)? {
            if let Some(word) = reader
                .move_to()
                .word(&[&words::CMP_TRUE, &words::CMP_FALSE])
            {
                if reader.rest().trim().is_empty() {
                    Err(E::NoValueAfterComparing)
                } else {
                    let mut value = reader.move_to().end().trim().to_string();
                    let mut token = reader.token()?;
                    if let Some(serialized) = token.bound.group().closed(&chars::QUOTES) {
                        value = serialized;
                    }
                    Ok(Some(VariableComparing {
                        name,
                        cmp: if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value,
                        token: close(reader),
                    }))
                }
            } else {
                reader.state().restore()?;
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for VariableComparing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.cmp, self.value)
    }
}

impl Operator for VariableComparing {
    fn process<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async move {
            let value = self
                .name
                .process(owner, components, args, cx)
                .await?
                .ok_or(operator::E::VariableIsNotAssigned(self.name.name.clone()))?
                .get_as_string()
                .ok_or(operator::E::FailToGetValueAsString)?;
            Ok(Some(AnyValue::new(match self.cmp {
                Cmp::Equal => value == self.value,
                Cmp::NotEqual => value != self.value,
            })))
        })
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        inf::tests::*,
        reader::entry::{
            embedded::If::Cmp, variable_comparing::VariableComparing, variable_name::VariableName,
        },
    };
    use proptest::prelude::*;

    impl Arbitrary for VariableComparing {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            (
                Cmp::arbitrary_with(scope.clone()),
                VariableName::arbitrary(),
                "[a-z][a-z0-9]*".prop_map(String::from),
            )
                .prop_map(|(cmp, name, value)| VariableComparing {
                    cmp,
                    value,
                    name,
                    token: 0,
                })
                .boxed()
        }
    }
}
