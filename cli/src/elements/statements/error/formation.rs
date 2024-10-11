use crate::{
    elements::{ElementRef, Error},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Error {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::ERROR,
            self.output.format(cursor)
        )
    }
}
