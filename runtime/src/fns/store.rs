use crate::*;

#[derive(Debug, Default)]
pub struct Fns {
    pub path: Vec<String>,
    pub funcs: HashMap<String, FnEntity>,
}

impl Fns {
    pub fn enter<S: AsRef<str>>(&mut self, mod_name: S) {
        self.path.push(mod_name.as_ref().to_owned());
    }
    pub fn leave(&mut self) {
        let _ = self.path.pop();
    }
    pub fn add<S: AsRef<str>>(&mut self, fn_name: S, entity: FnEntity) -> Result<(), E> {
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
    pub fn lookup<S: AsRef<str>>(&self, fn_name: S) -> Option<&FnEntity> {
        self.funcs
            .get(fn_name.as_ref())
            .or_else(|| self.funcs.get(self.fullname(fn_name).as_str()))
    }
    pub fn get_mut<S: AsRef<str>>(&mut self, fn_name: S) -> Option<&mut FnEntity> {
        if self.funcs.contains_key(fn_name.as_ref()) {
            self.funcs.get_mut(fn_name.as_ref())
        } else {
            self.funcs.get_mut(self.fullname(&fn_name).as_str())
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
