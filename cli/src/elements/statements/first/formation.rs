use crate::{
    elements::{ElementRef, First},
    inf::{Formation, FormationCursor},
};

impl Formation for First {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::First));
        format!(
            "{}first {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            self.block.format(&mut inner)
        )
    }
}
