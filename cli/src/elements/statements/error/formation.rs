use crate::{
    elements::{ElementId, Error},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Error {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{} {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            words::ERROR,
            self.output.format(cursor)
        )
    }
}
