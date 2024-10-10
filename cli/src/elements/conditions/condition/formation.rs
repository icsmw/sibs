use crate::{
    elements::{Condition, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Condition {
    fn elements_count(&self) -> usize {
        self.subsequence.elements_count()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        if self.elements_count() > cursor.max_elements()
            || self.to_string().len() > cursor.max_len()
        {
            format!(
                "{}({})",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                self.subsequence
                    .format(&mut cursor.reown(Some(ElementRef::Condition)))
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
