use crate::{
    elements::{ElementId, VariableAssignation},
    inf::{Formation, FormationCursor},
};

impl Formation for VariableAssignation {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::VariableAssignation));
        format!(
            "{}{} = {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.variable.format(&mut inner),
            self.assignation.format(&mut inner)
        )
    }
}
