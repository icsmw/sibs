use crate::*;

#[derive(Debug, Default)]
pub struct CFns {
    pub funcs: HashMap<Uuid, ClosureFnEntity>,
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
    pub fn lookup(&mut self, uuid: &Uuid) -> Option<&ClosureFnEntity> {
        self.funcs.get(uuid)
    }
}
