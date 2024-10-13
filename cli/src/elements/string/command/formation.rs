use crate::{
    elements::{Command, Element, ElementId},
    inf::{Formation, FormationCursor},
};

impl Formation for Command {
    fn elements_count(&self) -> usize {
        self.elements.len()
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut inner = cursor.reown(Some(ElementId::PatternString));
        if self.to_string().len() > cursor.max_len()
            || self.elements.len() > cursor.max_inline_injections()
        {
            format!(
                "{}`{}`",
                cursor.offset_as_string_if(&[ElementId::Block]),
                self.elements
                    .iter()
                    .map(|el| {
                        if let Element::SimpleString(el, _) = el {
                            el.format(&mut inner)
                        } else {
                            format!(
                                "{{\n{}{}\n{}}}",
                                inner.right().offset_as_string(),
                                el.format(&mut inner.right()),
                                inner.offset_as_string()
                            )
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )
        } else {
            format!("{}{self}", cursor.offset_as_string_if(&[ElementId::Block]),)
        }
    }
}
