use crate::{
    elements::{ElementRef, While},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for While {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::While));
        format!(
            "{}{} {} {}",
            cursor.offset_as_string_if(&[ElementRef::Block]),
            words::WHILE,
            self.condition,
            self.block.format(&mut inner)
        )
    }
}
