use crate::{
    elements::{Each, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Each {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}each({}; {}) {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.variable,
            self.input,
            self.block.format(cursor)
        )
    }
}
