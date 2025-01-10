use std::collections::hash_map;

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
        if let hash_map::Entry::Vacant(en) = self.table.entry(task.fullname()) {
            en.insert(task);
            Ok(())
        } else {
            Err(E::TaskDuplicate)
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
    pub(crate) fn lookup_by_caller(&self, caller: &Uuid) -> Option<&TaskEntity> {
        let name = self.links.get(caller)?;
        self.table.get(name)
    }
    pub async fn execute(
        &self,
        uuid: &Uuid,
        rt: Runtime,
        args: Vec<FnArgValue>,
    ) -> Result<RtValue, LinkedErr<E>> {
        let Some(entity) = self.lookup_by_caller(uuid) else {
            return Err(LinkedErr::unlinked(E::NoLinkedFunctions(*uuid)));
        };
        entity.execute(rt, args).await
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
