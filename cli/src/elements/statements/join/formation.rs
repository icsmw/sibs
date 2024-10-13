use crate::{
    elements::{ElementId, Join},
    inf::{Formation, FormationCursor},
};

impl Formation for Join {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Join));
        format!(
            "{}join {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.elements.format(&mut inner)
        )
    }
}
