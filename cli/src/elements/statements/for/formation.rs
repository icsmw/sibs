use crate::{
    elements::{ElementId, For},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for For {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::For));
        format!(
            "{}{} {} in {} {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            words::FOR,
            self.index,
            self.target,
            self.block.format(&mut inner)
        )
    }
}
