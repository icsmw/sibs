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
    pub fn new<S: ToString>(alias: S, parent: Option<Uuid>, visibility: JobVisibility) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            parent,
            alias: alias.to_string(),
            visibility,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    pub fn visibility(&self) -> &JobVisibility {
        &self.visibility
    }

    pub fn alias(&self) -> &str {
        &self.alias
    }
}

/// A lightweight element of a filtered job path, ordered from root to target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobElement {
    pub uuid: Uuid,
    pub alias: String,
}

impl From<&JobIdentity> for JobElement {
    fn from(identity: &JobIdentity) -> Self {
        Self {
            uuid: identity.uuid(),
            alias: identity.alias().to_owned(),
        }
    }
}
