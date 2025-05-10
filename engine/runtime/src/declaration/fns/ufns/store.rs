use crate::*;

#[derive(Debug, Default)]
pub struct UFns {
    pub path: Vec<String>,
    pub funcs: HashMap<String, UserFnEntity>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ String }` - function's name;
    pub links: HashMap<Uuid, String>,
}

impl UFns {
    pub fn enter<S: AsRef<str>>(&mut self, mod_name: S) {
        self.path.push(mod_name.as_ref().to_owned());
    }
    pub fn leave(&mut self) {
        let _ = self.path.pop();
    }
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, mut entity: UserFnEntity) -> Result<(), E> {
        let name = self.fullname(fn_name.as_ref());
        entity.fullname = self.fullname(fn_name);
        entity.verify(&name)?;
        if self.funcs.contains_key(&name) {
            return Err(E::FuncAlreadyRegistered(name));
        }
        self.funcs.insert(name, entity);
        Ok(())
    }
    pub fn set_result_ty<S: AsRef<str>>(&mut self, fn_name: S, ty: Ty) -> Result<(), E> {
        let name = self.fullname(fn_name);
        let Some(en) = self.funcs.get_mut(&name) else {
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
        let name = self.link(fn_name, caller)?;
        self.funcs.get(&name)
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
    fn link<S: AsRef<str>>(&mut self, fn_name: S, caller: &Uuid) -> Option<String> {
        if let Some(name) = if self.funcs.contains_key(fn_name.as_ref()) {
            Some(fn_name.as_ref().to_owned())
        } else if self
            .funcs
            .contains_key(self.fullname(fn_name.as_ref()).as_str())
        {
            Some(self.fullname(fn_name))
        } else {
            None
        } {
            self.links.insert(*caller, name.clone());
            Some(name)
        } else {
            None
        }
    }
    pub fn collect_by_path(&self, path: &[&str]) -> Vec<(String, &UserFnEntity)> {
        let prefix = path.join("::");
        self.funcs
            .iter()
            .filter(|(fullname, _entry)| prefix.is_empty() || fullname.starts_with(&prefix))
            .map(|(fullname, entry)| {
                if prefix.is_empty() {
                    (fullname.to_owned(), entry)
                } else {
                    let mut parts = fullname.split("::").collect::<Vec<&str>>();
                    parts.drain(..prefix.split("::").count());
                    (parts.join("::"), entry)
                }
            })
            .collect()
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
