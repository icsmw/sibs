use crate::{
    elements::{Comparing, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Comparing {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self
        )
    }
}
