use crate::{
    elements::{Comparing, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Comparing {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}
