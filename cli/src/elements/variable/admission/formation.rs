use crate::{
    elements::{ElementId, VariableName},
    inf::{Formation, FormationCursor},
};

impl Formation for VariableName {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{self}", cursor.offset_as_string_if(&[ElementId::Block]))
    }
}
