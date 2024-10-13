use crate::{
    elements::{ElementId, SimpleString},
    inf::{Formation, FormationCursor},
};

impl Formation for SimpleString {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self
        )
    }
}
