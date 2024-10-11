use crate::{
    elements::{ElementRef, Loop},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Loop {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Loop));
        format!(
            "{}{} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::LOOP,
            self.block.format(&mut inner)
        )
    }
}
