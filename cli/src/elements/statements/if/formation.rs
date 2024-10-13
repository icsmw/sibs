use crate::{
    elements::{ElementId, If},
    inf::{Formation, FormationCursor},
};

impl Formation for If {
    fn elements_count(&self) -> usize {
        self.threads.iter().map(|th| th.elements_count()).sum()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::If));
        format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.threads
                .iter()
                .map(|el| el.format(&mut inner))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
