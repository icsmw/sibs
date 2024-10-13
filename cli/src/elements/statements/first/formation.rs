use crate::{
    elements::{ElementId, First},
    inf::{Formation, FormationCursor},
};

impl Formation for First {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::First));
        format!(
            "{}first {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.block.format(&mut inner)
        )
    }
}
