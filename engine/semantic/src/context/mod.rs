use crate::*;

#[derive(Debug)]
pub struct SemanticCx {
    pub tys: TyStore,
    pub fns: Fns,
    pub tasks: Tasks,
    pub table: TypesTable,
    pub errs: Errors<E>,
    resilience: bool,
}

impl SemanticCx {
    pub fn new(resilience: bool) -> Self {
        Self {
            tys: TyStore::default(),
            fns: Fns::default(),
            tasks: Tasks::default(),
            table: TypesTable::default(),
            errs: Errors::default(),
            resilience,
        }
    }
    pub fn is_resilience(&self) -> bool {
        self.resilience
    }
    pub fn lookup_fn<S: AsRef<str>>(
        &mut self,
        name: S,
        caller: &Uuid,
    ) -> Result<Option<FnEntity<'_>>, E> {
        let uuid = if let Some(ty) = self.tys.lookup(name.as_ref())? {
            if let Some(Ty::Determined(DeterminedTy::Closure(uuid, ..))) = &ty.assigned {
                Some(*uuid)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(uuid) = uuid {
            return Ok(self.fns.lookup_by_uuid(&uuid, caller));
        }
        if let Some(entity) = self.fns.lookup(name.as_ref(), caller) {
            return Ok(Some(entity));
        }
        Ok(None)
    }
    pub fn lookup_fn_by_inps<S: AsRef<str>>(
        &mut self,
        name: S,
        incomes: &[&Ty],
        caller: &Uuid,
    ) -> Option<FnEntity> {
        self.fns.lookup_by_inps(name, incomes, caller)
    }
    pub fn lookup_task<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<&TaskEntity> {
        self.tasks.lookup(name.as_ref(), caller)
    }
    pub fn by_node<N: InferType + Identification>(&mut self, node: &N) -> Result<(), LinkedErr<E>> {
        if self.table.has(node.uuid()) {
            // It's PPM and it's already registred
            return Ok(());
        }
        let ty = node.infer_type(self)?;
        self.table.set(node.uuid(), ty);
        Ok(())
    }
    pub fn register(&mut self, uuid: &Uuid, ty: &Ty) {
        self.table.set(uuid, ty.clone());
    }
}
