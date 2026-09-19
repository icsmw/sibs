use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct JobIdentity {
    uuid: Uuid,
    parent: Option<Uuid>,
    alias: String,
}

impl JobIdentity {
    pub fn new<S: ToString>(alias: S, parent: Option<Uuid>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            parent,
            alias: alias.to_string(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent.clone()
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}
