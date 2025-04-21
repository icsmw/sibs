use crate::*;

pub struct ComponentAction {
    pub name: String,
}

impl ComponentAction {
    pub fn new(args: &mut Vec<String>) -> Result<Self, E> {
        if args.is_empty() {
            return Err(E::FailToGetComponentName);
        }
        Ok(Self {
            name: args.remove(0),
        })
    }
}

impl ActionMethods for ComponentAction {
    fn artifact(&self, _actions: &[Action]) -> Result<Vec<ActionArtifact>, E> {
        Ok(vec![ActionArtifact::Component(self.name.clone())])
    }
}
