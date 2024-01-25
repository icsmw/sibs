use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        entry::{Cmp, Component, Reader, Reading, VariableName},
        words, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub struct VariableComparing {
    pub name: VariableName,
    pub cmp: Cmp,
    pub value: String,
    pub token: usize,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, E> {
        reader.state().set();
        if let Some(name) = VariableName::read(reader)? {
            if let Some(word) = reader
                .move_to()
                .word(&[&words::CMP_TRUE, &words::CMP_FALSE])
            {
                if reader.rest().trim().is_empty() {
                    Err(E::NoValueAfterComparing)
                } else {
                    Ok(Some(VariableComparing {
                        name,
                        cmp: if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value: reader.move_to().end().trim().to_string(),
                        token: reader.token()?.id,
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
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async {
            let value = self
                .name
                .process(components, args, cx)
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
