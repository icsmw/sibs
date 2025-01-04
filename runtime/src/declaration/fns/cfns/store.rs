use crate::*;

#[derive(Debug, Default)]
pub struct CFns {
    pub funcs: HashMap<Uuid, ClosureFnEntity>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ Uuid }` - closure uuid;
    pub links: HashMap<Uuid, Uuid>,
}

impl CFns {
    pub fn add(&mut self, entity: ClosureFnEntity) -> Result<(), E> {
        entity.verify()?;
        if self.funcs.contains_key(&entity.uuid) {
            return Err(E::ClosureAlreadyRegistered(entity.uuid));
        }
        self.funcs.insert(entity.uuid, entity);
        Ok(())
    }
    pub fn set_result_ty(&mut self, uuid: &Uuid, ty: Ty) -> Result<(), E> {
        let Some(en) = self.funcs.get_mut(uuid) else {
            return Err(E::ClosureNotFound(*uuid));
        };
        en.result = ty;
        Ok(())
    }
    pub fn lookup(&mut self, uuid: &Uuid, caller: &Uuid) -> Option<&ClosureFnEntity> {
        let uuid = self.link(uuid, caller)?;
        self.funcs.get(&uuid)
    }
    pub(crate) fn lookup_by_caller(&self, caller: &Uuid) -> Option<&ClosureFnEntity> {
        let uuid = self.links.get(caller)?;
        self.funcs.get(uuid)
    }
    fn link(&mut self, cl_uuid: &Uuid, caller: &Uuid) -> Option<Uuid> {
        if !self.funcs.contains_key(cl_uuid) {
            None
        } else {
            self.links.insert(*caller, *cl_uuid);
            Some(*cl_uuid)
        }
    }
}
