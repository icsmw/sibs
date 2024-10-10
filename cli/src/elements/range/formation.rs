use crate::{
    elements::Range,
    inf::{Formation, FormationCursor},
};

impl Formation for Range {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!("{}..{}", self.from, self.to)
    }
}
