use crate::{
    elements::{ElementRef, Integer},
    inf::{Formation, FormationCursor},
};

impl Formation for Integer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}
