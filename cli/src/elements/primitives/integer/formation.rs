use crate::{
    elements::{ElementId, Integer},
    inf::{Formation, FormationCursor},
};

impl Formation for Integer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self
        )
    }
}
