use crate::{
    elements::Call,
    inf::{Formation, FormationCursor},
};

impl Formation for Call {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        format!(".{}", self.func)
    }
}
