use crate::{
    elements::{ElementId, Return},
    inf::{Formation, FormationCursor},
    reader::words,
};

impl Formation for Return {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!(
            "{}{}{}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            words::RETURN,
            if let Some(el) = self.output.as_ref() {
                format!(" {}", el.format(cursor))
            } else {
                String::new()
            }
        )
    }
}
