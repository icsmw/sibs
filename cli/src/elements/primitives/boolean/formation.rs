use crate::{
    elements::{Boolean, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Boolean {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self
        )
    }
}
