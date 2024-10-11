use crate::{
    elements::{ElementRef, For},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for For {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::For));
        format!(
            "{}{} {} in {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::FOR,
            self.index,
            self.target,
            self.block.format(&mut inner)
        )
    }
}
