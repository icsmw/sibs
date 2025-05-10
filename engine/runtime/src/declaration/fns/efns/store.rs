use crate::*;

#[derive(Debug, Default)]
pub struct EFns {
    pub funcs: HashMap<String, EmbeddedFnEntity>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ String }` - function's name;
    pub links: HashMap<Uuid, String>,
}

impl EFns {
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, entity: EmbeddedFnEntity) -> Result<(), E> {
        entity.verify()?;
        if self.funcs.contains_key(fn_name.as_ref()) {
            return Err(E::FuncAlreadyRegistered(fn_name.as_ref().to_owned()));
        }
        self.funcs.insert(fn_name.as_ref().to_owned(), entity);
        Ok(())
    }
    pub(crate) fn lookup<S: AsRef<str>>(
        &mut self,
        fn_name: S,
        caller: &Uuid,
    ) -> Option<&EmbeddedFnEntity> {
        let name = self.link(fn_name.as_ref(), caller)?;
        self.funcs.get(&name)
    }
    pub(crate) fn lookup_by_inps<S: AsRef<str>>(
        &mut self,
        name: S,
        incomes: &[&Ty],
        caller: &Uuid,
    ) -> Option<&EmbeddedFnEntity> {
        let filtered = self
            .funcs
            .values()
            .filter(|en| en.name == name.as_ref() && en.compatible(incomes))
            .map(|en| en.fullname.to_owned())
            .collect::<Vec<String>>();
        if filtered.len() != 1 {
            None
        } else {
            filtered.first().and_then(|name| {
                let _ = self.link(name, caller);
                self.funcs.get(name)
            })
        }
    }
    pub fn collect(&self) -> Vec<(String, &EmbeddedFnEntity)> {
        self.funcs
            .iter()
            .map(|(name, entry)| (name.to_owned(), entry))
            .collect()
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
