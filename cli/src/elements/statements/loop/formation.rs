use crate::{
    elements::{ElementId, Loop},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Loop {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Loop));
        format!(
            "{}{} {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            words::LOOP,
            self.block.format(&mut inner)
        )
    }
}
