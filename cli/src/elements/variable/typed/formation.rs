use crate::{
    elements::VariableType,
    inf::{Formation, FormationCursor},
};

impl Formation for VariableType {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}
