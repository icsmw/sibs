use crate::{
    elements::Meta,
    inf::{Formation, FormationCursor},
};

impl Formation for Meta {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        self.inner
            .iter()
            .map(|v| format!("{}/// {v}", cursor.offset_as_string()))
            .collect::<Vec<String>>()
            .join("\n")
            .to_string()
    }
}
