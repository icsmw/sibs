use crate::{
    elements::Combination,
    inf::{Formation, FormationCursor},
};

impl Formation for Combination {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}
