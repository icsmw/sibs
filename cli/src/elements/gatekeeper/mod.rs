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
    elements::{Element, Reference},
    error::LinkedErr,
    inf::{operator::E, Execute, ExecuteContext},
};

#[derive(Debug, Clone)]
pub struct Gatekeeper {
    pub function: Box<Element>,
    pub refs: Box<Element>,
    pub token: usize,
}

impl Gatekeeper {
    pub fn get_refs(&self) -> Vec<&Reference> {
        let mut refs = Vec::new();
        let Element::Values(values, _) = self.refs.as_ref() else {
            unreachable!("References can be stored only in Values of Gatekeeper")
        };
        for el in values.elements.iter() {
            let Element::Reference(reference, _) = el else {
                unreachable!("Only references can be stored in Gatekeeper")
            };
            refs.push(reference);
        }
        refs
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn skippable<'a>(
        gatekeepers: Vec<&Element>,
        task_ref: &Reference,
        cx: ExecuteContext<'a>,
    ) -> Result<bool, LinkedErr<E>> {
        if gatekeepers.is_empty() {
            return Ok(false);
        }
        for el in gatekeepers.iter() {
            let Element::Gatekeeper(gatekeeper, _) = el else {
                continue;
            };
            let refs = gatekeeper.get_refs();
            if !refs.is_empty() && !refs.iter().any(|reference| reference == &task_ref) {
                return Ok(false);
            }
            // On "true" - task should be done; on "false" - can be skipped.
            if el
                .execute(cx.clone().args(&[]))
                .await?
                .as_bool()
                .ok_or(E::NoBoolValueFromGatekeeper)?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
