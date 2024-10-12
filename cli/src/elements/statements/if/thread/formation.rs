use crate::{
    elements::{ElementRef, IfThread},
    inf::{Formation, FormationCursor},
};

impl Formation for IfThread {
    fn elements_count(&self) -> usize {
        match self {
            Self::If(el, _) => el.elements_count(),
            Self::Else(_) => 0,
        }
    }
    fn format(&self, cursor: &mut FormationCursor) -> String {
        match self {
            Self::If(el, block) => format!(
                "{}if {} {}",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                el.format(cursor),
                block.format(cursor)
            ),
            Self::Else(block) => format!(
                "{}else {}",
                cursor.offset_as_string_if(&[ElementRef::Block]),
                block.format(cursor)
            ),
        }
    }
}
