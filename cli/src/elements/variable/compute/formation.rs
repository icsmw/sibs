use crate::{
    elements::{Compute, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Compute {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Compute));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.left.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}
