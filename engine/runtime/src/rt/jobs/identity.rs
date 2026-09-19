use uuid::Uuid;

#[derive(Default, Debug, Clone)]
pub enum JobVisibility {
    #[default]
    Hidden,
    Visible,
}

#[derive(Debug, Clone)]
pub struct JobIdentity {
    uuid: Uuid,
    parent: Option<Uuid>,
    alias: String,
    visibility: JobVisibility,
}

impl JobIdentity {
    pub fn new<S: ToString>(alias: S, parent: Option<Uuid>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            parent,
            alias: alias.to_string(),
            visibility: JobVisibility::default(),
        }
    }

    pub fn visible<S: ToString>(alias: S, parent: Option<Uuid>) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            parent,
            alias: alias.to_string(),
            visibility: JobVisibility::Visible,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}
