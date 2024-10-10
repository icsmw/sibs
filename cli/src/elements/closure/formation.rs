use crate::{
    elements::{Closure, ElementRef},
    inf::{Formation, FormationCursor},
};

impl Formation for Closure {
    fn elements_count(&self) -> usize {
        self.args.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let output = format!(
            "{}{}",
            cursor.offset_as_string_if(&[ElementRef::Block, ElementRef::Component]),
            self
        );
        format!(
            "{output}{}",
            if cursor.parent.is_none() { ";\n" } else { "" }
        )
    }
}
