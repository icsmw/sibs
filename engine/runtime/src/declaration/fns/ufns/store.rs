use crate::*;

#[derive(Debug, Default)]
pub struct UFns {
    path: Vec<String>,
    /// Each function is stored once, independently of the paths that expose it.
    funcs: HashMap<Uuid, UserFnEntity>,
    aliases: HashMap<String, Uuid>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ Uuid }` - function's uuid;
    links: HashMap<Uuid, Uuid>,
}

impl UFns {
    pub fn get_funcs(&self) -> &HashMap<Uuid, UserFnEntity> {
        &self.funcs
    }
    pub fn get_funcs_mut(&mut self) -> &mut HashMap<Uuid, UserFnEntity> {
        &mut self.funcs
    }
    pub fn get_links(&self) -> &HashMap<Uuid, Uuid> {
        &self.links
    }
    pub fn enter<S: AsRef<str>>(&mut self, mod_name: S) {
        self.path.push(mod_name.as_ref().to_owned());
    }
    pub fn leave(&mut self) {
        let _ = self.path.pop();
    }
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, entity: UserFnEntity) -> Result<(), E> {
        let name = self.fullname(fn_name.as_ref());
        entity.verify(&name)?;
        if self
            .aliases
            .get(&name)
            .is_some_and(|previous| previous != &entity.uuid)
        {
            return Err(E::FuncAlreadyRegistered(name.to_owned()));
        }
        let uuid = entity.uuid;
        self.funcs.entry(uuid).or_insert(entity);
        self.aliases.insert(name, uuid);
        Ok(())
    }
    pub fn find<S: AsRef<str>>(&self, name: S) -> Option<&UserFnEntity> {
        self.funcs.get(self.aliases.get(name.as_ref())?)
    }
    pub fn set_result_ty<S: AsRef<str>>(&mut self, fn_name: S, ty: Ty) -> Result<(), E> {
        let name = self.fullname(fn_name);
        let Some(uuid) = self.aliases.get(&name) else {
            return Err(E::FuncNotFound(name));
        };
        let Some(en) = self.funcs.get_mut(uuid) else {
            return Err(E::FuncNotFound(name));
        };
        en.result = ty;
        Ok(())
    }
    pub(crate) fn lookup<S: AsRef<str>>(
        &mut self,
        fn_name: S,
        caller: &Uuid,
    ) -> Option<&UserFnEntity> {
        let uuid = self.link(fn_name, caller)?;
        self.funcs.get(&uuid)
    }
    pub(crate) fn lookup_by_inps<S: AsRef<str>>(
        &mut self,
        name: S,
        incomes: &[&Ty],
        caller: &Uuid,
    ) -> Option<&UserFnEntity> {
        let filtered = self
            .funcs
            .values()
            .filter(|en| en.name == name.as_ref() && en.compatible(incomes))
            .map(|en| en.uuid)
            .collect::<Vec<_>>();
        if filtered.len() != 1 {
            None
        } else {
            filtered.first().and_then(|uuid| {
                self.links.insert(*caller, *uuid);
                self.funcs.get(uuid)
            })
        }
    }
    pub fn collect_by_path(&self, path: &[&str]) -> Vec<(String, &UserFnEntity)> {
        let prefix = if path.is_empty() {
            String::new()
        } else {
            format!("{}::", path.join("::"))
        };
        self.aliases
            .iter()
            .filter_map(|(fullname, uuid)| {
                let name = fullname.strip_prefix(&prefix)?;
                Some((name.to_owned(), self.funcs.get(uuid)?))
            })
            .collect()
    }

    fn link<S: AsRef<str>>(&mut self, fn_name: S, caller: &Uuid) -> Option<Uuid> {
        let uuid = *self
            .aliases
            .get(fn_name.as_ref())
            .or_else(|| self.aliases.get(&self.fullname(fn_name.as_ref())))?;
        self.links.insert(*caller, uuid);
        Some(uuid)
    }
    fn fullname<S: AsRef<str>>(&self, fn_name: S) -> String {
        let path = self.path.join("::");
        format!(
            "{path}{}{}",
            if path.is_empty() { "" } else { "::" },
            fn_name.as_ref()
        )
    }
}
