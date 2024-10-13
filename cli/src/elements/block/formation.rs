use crate::{
    elements::{Block, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Block {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Block)).right();
        format!(
            "{{\n{}{}{}}}",
            self.elements
                .iter()
                .map(|el| format!("{};", el.format(&mut inner),))
                .collect::<Vec<String>>()
                .join("\n"),
            if self.elements.is_empty() { "" } else { "\n" },
            cursor.offset_as_string()
        )
    }
}
