use crate::{
    elements::{ElementRef, Task},
    inf::{Formation, FormationCursor},
};

impl Formation for Task {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementRef::Task));
        format!(
            "@{}{}{}{} {}",
            cursor.offset_as_string(),
            self.name.value,
            if self.declarations.is_empty() && self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.declarations
                        .iter()
                        .map(|d| d.format(&mut inner))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            },
            if self.dependencies.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    self.dependencies
                        .iter()
                        .map(|d| d.format(&mut inner))
                        .collect::<Vec<String>>()
                        .join(";")
                )
            },
            self.block.format(&mut inner)
        )
    }
}
