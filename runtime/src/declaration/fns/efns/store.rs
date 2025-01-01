use crate::*;

#[derive(Debug, Default)]
pub struct EFns {
    pub path: Vec<String>,
    pub funcs: HashMap<String, EmbeddedFnEntity>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ String }` - function's name;
    pub links: HashMap<Uuid, String>,
}

impl EFns {
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, entity: EmbeddedFnEntity) -> Result<(), E> {
        if self.funcs.contains_key(fn_name.as_ref()) {
            return Err(E::FuncAlreadyRegistered(fn_name.as_ref().to_owned()));
        }
        self.funcs.insert(fn_name.as_ref().to_owned(), entity);
        Ok(())
    }
    pub fn lookup<S: AsRef<str>>(
        &mut self,
        fn_name: S,
        caller: &Uuid,
    ) -> Option<&EmbeddedFnEntity> {
        let name = self.link(fn_name, caller)?;
        self.funcs.get(&name)
    }
    pub fn lookup_by_caller(&self, caller: &Uuid) -> Option<&EmbeddedFnEntity> {
        let name = self.links.get(caller)?;
        self.funcs.get(name)
    }

    fn link<S: AsRef<str>>(&mut self, fn_name: S, caller: &Uuid) -> Option<String> {
        if let Some(name) = if self.funcs.contains_key(fn_name.as_ref()) {
            Some(fn_name.as_ref().to_owned())
        } else {
            None
        } {
            self.links.insert(*caller, name.clone());
            Some(name)
        } else {
            None
        }
    }
}
