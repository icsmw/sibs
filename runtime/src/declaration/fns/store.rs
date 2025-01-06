use crate::*;

#[derive(Debug, Default)]
pub struct Fns {
    pub efns: EFns,
    pub ufns: UFns,
    pub cfns: CFns,
}

impl Fns {
    pub fn lookup<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(entity) = self.ufns.lookup(name.as_ref(), caller) {
            Some(FnEntity::UFn(entity))
        } else {
            self.efns.lookup(name, caller).map(FnEntity::EFn)
        }
    }
    pub fn lookup_by_caller(&self, caller: &Uuid) -> Option<FnEntity<'_>> {
        if let Some(name) = self.ufns.links.get(caller) {
            self.ufns.funcs.get(name).map(FnEntity::UFn)
        } else if let Some(name) = self.efns.links.get(caller) {
            self.efns.funcs.get(name).map(FnEntity::EFn)
        } else if let Some(uuid) = self.cfns.links.get(caller) {
            self.cfns.funcs.get(uuid).map(FnEntity::CFn)
        } else {
            None
        }
    }
    pub fn lookup_by_uuid(&mut self, uuid: &Uuid, caller: &Uuid) -> Option<FnEntity<'_>> {
        self.cfns.lookup(uuid, caller).map(FnEntity::CFn)
    }
    pub fn lookup_closure(&self, uuid: &Uuid) -> Option<FnEntity<'_>> {
        self.cfns.funcs.get(uuid).map(FnEntity::CFn)
    }
    /// Asynchronously executes a function in the runtime with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `uuid` - A reference to a `Uuid` of caller node.
    /// * `rt` - The runtime environment in which the function will be executed.
    /// * `args` - A vector of `RtValue` containing the arguments to pass to the function.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `RtValue` - The result of the executed function.
    /// * `E` - An error if the function execution fails.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * Sending the execution demand to the runtime fails.
    /// * Awaiting the response from the runtime fails.
    pub async fn execute(
        &self,
        uuid: &Uuid,
        rt: Runtime,
        args: Vec<FnArgValue>,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        let Some(fn_entity) = self
            .lookup_by_caller(uuid)
            .or_else(|| self.lookup_closure(uuid))
        else {
            return Err(LinkedErr::unlinked(E::NoLinkedFunctions(*uuid)));
        };
        fn_entity.execute(rt, args, self, caller).await
    }
}
