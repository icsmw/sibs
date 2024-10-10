mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{
    elements::{Boolean, Element, Integer, Metadata, PatternString, Reference, SimpleString},
    error::LinkedErr,
    inf::{operator::E, ExecuteContext, Value},
};

#[derive(Debug, Clone)]
pub struct Task {
    // TODO: replace SimpleString with Element
    pub name: SimpleString,
    pub declarations: Vec<Element>,
    pub dependencies: Vec<Element>,
    pub block: Box<Element>,
    pub token: usize,
}

impl Task {
    pub fn get_name(&self) -> &str {
        &self.name.value
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn get_args_values<'a>(
        &'a self,
        cx: ExecuteContext<'a>,
    ) -> Result<Vec<Value>, LinkedErr<E>> {
        if self.declarations.len() != cx.args.len() {
            Err(E::DismatchTaskArgumentsCount(
                self.declarations.len(),
                self.declarations
                    .iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                cx.args.len(),
                cx.args
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .by(self))?;
        }
        let mut values = Vec::new();
        for (i, el) in self.declarations.iter().enumerate() {
            if let Element::VariableDeclaration(declaration, _) = el {
                values.push(
                    declaration
                        .get_val(cx.clone().args(&[cx.args[i].to_owned()]))
                        .await?,
                );
            } else {
                return Err(E::InvalidVariableDeclaration.by(self));
            }
        }
        Ok(values)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn as_reference<'a>(
        &'a self,
        cx: ExecuteContext<'a>,
    ) -> Result<Reference, LinkedErr<E>> {
        let mut inputs = Vec::new();
        for arg in self.get_args_values(cx).await?.into_iter() {
            if let Some(v) = arg.as_num() {
                inputs.push(Element::Integer(
                    Integer { value: v, token: 0 },
                    Metadata::empty(),
                ));
            } else if let Value::bool(v) = arg {
                inputs.push(Element::Boolean(
                    Boolean { value: v, token: 0 },
                    Metadata::empty(),
                ));
            } else if let Some(value) = arg.as_string() {
                inputs.push(Element::PatternString(
                    PatternString {
                        elements: vec![Element::SimpleString(
                            SimpleString { value, token: 0 },
                            Metadata::empty(),
                        )],
                        token: 0,
                    },
                    Metadata::empty(),
                ));
            } else {
                return Err(E::NoneStringTaskArgumentForReference.by(self));
            }
        }
        Ok(Reference {
            path: vec![self.get_name().to_owned()],
            inputs,
            token: 0,
        })
    }
}
