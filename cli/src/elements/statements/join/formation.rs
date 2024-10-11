use crate::{
    elements::{ElementRef, Join},
    inf::{Formation, FormationCursor},
};

impl Formation for Join {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Join));
        format!(
            "{}join {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.elements.format(&mut inner)
        )
    }
}
