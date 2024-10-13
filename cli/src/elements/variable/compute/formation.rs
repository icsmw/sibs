use crate::{
    elements::{Compute, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Compute {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Compute));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.left.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}
