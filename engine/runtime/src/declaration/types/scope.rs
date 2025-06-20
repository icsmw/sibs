use tracing::debug;

use crate::*;

#[derive(Debug, Default)]
pub struct TyScope {
    pub levels: HashMap<Uuid, HashMap<String, TypeEntity>>,
    pub location: Vec<Uuid>,
    pub parent: TyParent,
}

impl TyScope {
    pub fn enter(&mut self, uuid: &Uuid) {
        self.levels.entry(*uuid).or_default();
        self.location.push(*uuid);
    }
    pub fn leave(&mut self) -> Result<(), E> {
        if !self.location.is_empty() {
            let _ = self.location.pop();
            Ok(())
        } else {
            Err(E::AttemptToLeaveRootContextLevel)
        }
    }
    pub fn insert<S: AsRef<str>>(&mut self, name: S, edt: TypeEntity) -> Result<(), E> {
        if let Some(uuid) = self.location.last() {
            if let Some(sc) = self.levels.get_mut(uuid) {
                sc.insert(name.as_ref().to_owned(), edt);
                return Ok(());
            }
        }
        Err(E::NoCurrentContextLevel)
    }
    pub fn lookup<S: AsRef<str>>(&self, name: S) -> Option<&TypeEntity> {
        for uuid in self.location.iter().rev() {
            if let Some(edt) = self.levels.get(uuid)?.get(name.as_ref()) {
                return Some(edt);
            }
        }
        None
    }
    pub fn lookup_by_node(&self, node: &LinkedNode) -> Option<&Ty> {
        let name = match node.get_node() {
            Node::Expression(Expression::Variable(node)) => Some(&node.ident),
            _ => None,
        }?;
        for uuid in self.location.iter().rev() {
            if let Some(edt) = self.levels.get(uuid)?.get(name) {
                return edt.ty();
            }
        }
        None
    }
    pub fn find<S: AsRef<str>>(&self, name: S, location: &[Uuid]) -> Option<&TypeEntity> {
        for uuid in location.iter() {
            if let Some(edt) = self.levels.get(uuid)?.get(name.as_ref()) {
                return Some(edt);
            }
        }
        None
    }
    pub fn get_by_node(&self, uuid: &Uuid) -> Option<&Ty> {
        self.levels.iter().find_map(|(_, level)| {
            level.iter().find_map(|(_, entity)| {
                if &entity.node == uuid {
                    entity.ty()
                } else {
                    None
                }
            })
        })
    }
    pub fn get_all_variables(&self, location: &[Uuid]) -> Option<Vec<(&String, &TypeEntity)>> {
        let mut variables = Vec::new();
        for uuid in location.iter() {
            variables.extend(
                self.levels
                    .get(uuid)?
                    .iter()
                    .map(|(name, ty)| (name, ty))
                    .collect::<Vec<(&String, &TypeEntity)>>(),
            );
        }
        Some(variables)
    }
}
