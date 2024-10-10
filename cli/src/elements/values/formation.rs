use crate::{
    elements::{ElementRef, Values},
    inf::{Formation, FormationCursor},
};

impl Formation for Values {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.to_string().len() > cursor.max_len() && self.elements.len() > cursor.max_items() {
            format!(
                "{}(\n{}\n{})",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                self.elements
                    .iter()
                    .map(|v| format!(
                        "{}{}",
                        cursor.right().offset_as_string(),
                        v.format(&mut cursor.reown(Some(ElementRef::Values)).right())
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                cursor.offset_as_string_if(&[ElementRef::Block, ElementRef::Function])
            )
        } else {
            format!(
                "{}{}",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                self
            )
        }
    }
}
