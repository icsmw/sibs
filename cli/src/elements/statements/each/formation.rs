use crate::{
    elements::{Each, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Each {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}each({}; {}) {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.variable,
            self.input,
            self.block.format(cursor)
        )
    }
}
