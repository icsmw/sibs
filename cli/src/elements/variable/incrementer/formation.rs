use crate::{
    elements::{ElementRef, Incrementer},
    inf::{Formation, FormationCursor},
};

impl Formation for Incrementer {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Incrementer));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.variable.format(&mut inner),
            self.operator,
            self.right.format(&mut inner)
        )
    }
}
