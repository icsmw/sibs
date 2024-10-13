use crate::{
    elements::{ElementId, Gatekeeper},
    inf::{Formation, FormationCursor},
};

impl Formation for Gatekeeper {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Gatekeeper));
        format!(
            "{}{} -> {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.function.format(&mut inner),
            self.refs.format(&mut inner),
        )
    }
}
