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
        let path = self.path.join("::");
        let name = format!(
            "{path}{}{}",
            if path.is_empty() { "" } else { "::" },
            fn_name.as_ref()
        );
        if self.funcs.contains_key(&name) {
            return Err(E::FuncExists(name));
        }
        self.funcs.insert(name, entity);
        Ok(())
    }
}
