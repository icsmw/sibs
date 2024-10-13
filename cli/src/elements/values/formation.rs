use crate::{
    elements::{ElementId, Values},
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
                cursor.offset_as_string_if(&[ElementId::Block]),
                self.elements
                    .iter()
                    .map(|v| format!(
                        "{}{}",
                        cursor.right().offset_as_string(),
                        v.format(&mut cursor.reown(Some(ElementId::Values)).right())
                    ))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                cursor.offset_as_string_if(&[ElementId::Block, ElementId::Function])
            )
        } else {
            format!(
                "{}{}",
                cursor.offset_as_string_if(&[ElementId::Block]),
                self
            )
        }
    }
}
