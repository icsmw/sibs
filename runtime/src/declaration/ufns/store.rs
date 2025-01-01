use crate::*;

#[derive(Debug, Default)]
pub struct Fns {
    pub path: Vec<String>,
    pub funcs: HashMap<String, UserFnEntity>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ String }` - function's name;
    links: HashMap<Uuid, String>,
}

impl Fns {
    pub fn enter<S: AsRef<str>>(&mut self, mod_name: S) {
        self.path.push(mod_name.as_ref().to_owned());
    }
    pub fn leave(&mut self) {
        let _ = self.path.pop();
    }
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, entity: UserFnEntity) -> Result<(), E> {
        let name = self.fullname(fn_name);
        if self.funcs.contains_key(&name) {
            return Err(E::FuncAlreadyRegistered(name));
        }
        self.funcs.insert(name, entity);
        Ok(())
    }
    pub fn set_result_ty<S: AsRef<str>>(&mut self, fn_name: S, ty: DataType) -> Result<(), E> {
        let name = self.fullname(fn_name);
        let Some(en) = self.funcs.get_mut(&name) else {
            return Err(E::FuncNotFound(name));
        };
        en.result = ty;
        Ok(())
    }
    pub fn lookup<S: AsRef<str>>(&mut self, fn_name: S, caller: &Uuid) -> Option<&UserFnEntity> {
        let name = self.link(fn_name, caller)?;
        self.funcs.get(&name)
    }
    pub fn get_mut<S: AsRef<str>>(&mut self, fn_name: S) -> Option<&mut UserFnEntity> {
        if self.funcs.contains_key(fn_name.as_ref()) {
            self.funcs.get_mut(fn_name.as_ref())
        } else {
            self.funcs.get_mut(self.fullname(&fn_name).as_str())
        }
    }
    pub fn lookup_by_caller(&self, caller: &Uuid) -> Option<&UserFnEntity> {
        let name = self.links.get(caller)?;
        self.funcs.get(name)
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

    fn fullname<S: AsRef<str>>(&self, fn_name: S) -> String {
        let path = self.path.join("::");
        format!(
            "{path}{}{}",
            if path.is_empty() { "" } else { "::" },
            fn_name.as_ref()
        )
    }
}
