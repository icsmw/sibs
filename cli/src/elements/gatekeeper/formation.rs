use crate::{
    elements::{ElementRef, Gatekeeper},
    inf::{Formation, FormationCursor},
};

impl Formation for Gatekeeper {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Gatekeeper));
        format!(
            "{}{} -> {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.function.format(&mut inner),
            self.refs.format(&mut inner),
        )
    }
}
