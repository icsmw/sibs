use crate::*;

#[derive(Debug, Default)]
pub struct Tasks {
    pub table: HashMap<String, TaskEntity>,
    pub component: Option<MasterComponent>,
    /// Collected calls table
    /// * `{ Uuid }` - caller's node uuid;
    /// * `{ String }` - function's name;
    pub links: HashMap<Uuid, String>,
}

impl Tasks {
    pub fn add(&mut self, task: TaskEntity) -> Result<(), E> {
        let Some(component) = &self.component else {
            return Err(E::NoMasterComponent(task.name.clone()));
        };
        let name = format!("{}:{}", component.name, task.name);
        if self.table.contains_key(&task.name) {
            Err(E::TaskDuplicate)
        } else {
            self.table.insert(name, task);
            Ok(())
        }
    }
    pub fn master<S: AsRef<str>>(&mut self, name: S, uuid: &Uuid) {
        self.component = Some(MasterComponent {
            name: name.as_ref().to_owned(),
            uuid: *uuid,
        });
    }
    pub fn get_master(&self) -> Option<MasterComponent> {
        self.component.clone()
    }
    pub fn lookup<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<&TaskEntity> {
        let name = self.link(name, caller)?;
        self.table.get(&name)
    }
    fn link<S: AsRef<str>>(&mut self, name: S, caller: &Uuid) -> Option<String> {
        if let Some(name) = if self.table.contains_key(name.as_ref()) {
            Some(name.as_ref().to_owned())
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
