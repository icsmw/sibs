use crate::{
    elements::{ElementId, Incrementer},
    inf::{Formation, FormationCursor},
};

impl Formation for Incrementer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Incrementer));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.variable.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}
