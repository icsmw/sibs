use crate::{
    elements::{ElementRef, SimpleString},
    inf::{Formation, FormationCursor},
};

impl Formation for SimpleString {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self
        )
    }
}
