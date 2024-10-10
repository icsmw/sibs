use crate::{
    elements::{ElementRef, Reference},
    inf::{Formation, FormationCursor},
};

impl Formation for Reference {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}
