use crate::{
    elements::VariableVariants,
    inf::{Formation, FormationCursor},
};

impl Formation for VariableVariants {
    fn elements_count(&self) -> usize {
        self.values.len()
    }
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}
