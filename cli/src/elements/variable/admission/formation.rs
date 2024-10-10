use crate::{
    elements::{ElementRef, VariableName},
    inf::{Formation, FormationCursor},
};

impl Formation for VariableName {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{self}", cursor.offset_as_string_if(&[ElementRef::Block]))
    }
}
