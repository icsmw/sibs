use crate::{
    elements::{ElementId, Optional},
    inf::{Formation, FormationCursor},
};

impl Formation for Optional {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::Optional));
        format!(
            "{}{} => {}",
            cursor.offset_as_string_if(&[ElementId::Block]),
            self.condition.format(&mut inner),
            self.action.format(&mut inner),
        )
        // format!("{}{}", cursor.offset_as_string_if(&[ElementId::Block]), self)
    }
}
