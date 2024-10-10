use crate::{
    elements::{Boolean, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Boolean {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}
