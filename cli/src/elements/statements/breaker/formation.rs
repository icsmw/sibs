use crate::{
    elements::{Breaker, ElementRef},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Breaker {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::BREAK
        )
    }
}
