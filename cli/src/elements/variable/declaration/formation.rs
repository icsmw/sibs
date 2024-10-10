use crate::{
    elements::VariableDeclaration,
    inf::{Formation, FormationCursor},
};

impl Formation for VariableDeclaration {
    fn format(&self, _cursor: &mut FormationCursor) -> String {
        self.to_string()
    }
}
