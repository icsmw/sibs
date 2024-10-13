use crate::{
    elements::{Breaker, ElementId},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Breaker {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            words::BREAK
        )
    }
}
