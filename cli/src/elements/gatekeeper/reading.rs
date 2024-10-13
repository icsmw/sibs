use crate::{
    elements::{Element, ElementId, Gatekeeper},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Gatekeeper> for Gatekeeper {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        if reader.rest().trim().starts_with(words::REF_TO) {
            return Ok(None);
        }
        let close = reader.open_token(ElementId::Gatekeeper);
        let function = if let Some(el) = Element::include(reader, &[ElementId::Function])? {
            Box::new(el)
        } else {
            return Ok(None);
        };
        if !reader.rest().trim().starts_with(words::REF_TO) {
            return Ok(None);
        }
        if reader.move_to().expression(&[words::REF_TO]).is_none() {
            return Err(E::NoReferenceForGatekeeper.by_reader(reader));
        }
        let Some(refs) = Element::include(reader, &[ElementId::Values, ElementId::Reference])?
        else {
            return Err(E::NoReferenceForGatekeeper.by_reader(reader));
        };
        match &refs {
            Element::Reference(..) => {}
            Element::Values(values, _) => {
                if values
                    .elements
                    .iter()
                    .any(|el| !matches!(el, Element::Reference(..)))
                {
                    return Err(E::GatekeeperShouldRefToTask.by_reader(reader));
                }
            }
            _ => {
                return Err(E::GatekeeperShouldRefToTask.by_reader(reader));
            }
        }
        Ok(Some(Gatekeeper {
            token: close(reader),
            function,
            refs: Box::new(refs),
        }))
    }
}

impl Dissect<Gatekeeper, Gatekeeper> for Gatekeeper {}
