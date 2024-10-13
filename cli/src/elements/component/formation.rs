use crate::{
    elements::{Component, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Component {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Component)).right();
        format!(
            "#({}{})\n{}",
            self.name,
            self.cwd
                .as_ref()
                .map(|cwd| format!(": {}", cwd.display()))
                .unwrap_or_default(),
            self.elements
                .iter()
                .map(|el| format!("{};", el.format(&mut inner)))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
