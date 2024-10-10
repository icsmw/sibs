use crate::{
    elements::Accessor,
    inf::{Formation, FormationCursor},
};

impl Formation for Accessor {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!("[{}]", self.index)
    }
}
