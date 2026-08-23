use bstorage::Storage;

use crate::*;

pub struct ValueAccess<'a> {
    owner: &'a Uuid,
    cxs: &'a ExecutionContexts,
}

impl ValueAccess<'_> {
    pub async fn set_parent_vl(&self, vl: ParentValue) -> Result<(), E> {
        self.cxs.set_parent_vl(*self.owner, vl).await
    }

    pub async fn withdraw_parent_vl(&self) -> Result<Option<ParentValue>, E> {
        self.cxs.withdraw_parent_vl(*self.owner).await
    }

    pub async fn drop_parent_vl(&self) -> Result<(), E> {
        self.cxs.drop_parent_vl(*self.owner).await
    }

    pub async fn insert<S: ToString>(&self, name: S, vl: RtValue) -> Result<(), E> {
        self.cxs.insert(*self.owner, name, vl).await
    }

    pub async fn update<S: ToString>(&self, name: S, vl: RtValue) -> Result<(), E> {
        self.cxs.update(*self.owner, name, vl).await
    }

    pub async fn lookup<S: ToString>(&self, name: S) -> Result<Option<Arc<RtValue>>, E> {
        self.cxs.lookup(*self.owner, name).await
    }
}

pub struct ScopeAccess<'a> {
    owner: &'a Uuid,
    cxs: &'a ExecutionContexts,
}

impl ScopeAccess<'_> {
    pub async fn open(&self, uuid: &Uuid) -> Result<(), E> {
        self.cxs.open(*self.owner, uuid).await
    }

    pub async fn close(&self) -> Result<(), E> {
        self.cxs.close(*self.owner).await
    }

    pub async fn enter(&self, uuid: &Uuid) -> Result<(), E> {
        self.cxs.enter(*self.owner, uuid).await
    }

    pub async fn leave(&self) -> Result<(), E> {
        self.cxs.leave(*self.owner).await
    }
}

pub struct LoopAccess<'a> {
    owner: &'a Uuid,
    cxs: &'a ExecutionContexts,
}

impl LoopAccess<'_> {
    pub async fn open(&self, uuid: &Uuid) -> Result<(), E> {
        self.cxs.open_loop(*self.owner, uuid).await
    }

    pub async fn close(&self) -> Result<(), E> {
        self.cxs.close_loop(*self.owner).await
    }

    pub async fn set_break(&self) -> Result<(), E> {
        self.cxs.set_break(*self.owner).await
    }

    pub async fn is_stopped(&self) -> Result<bool, E> {
        self.cxs.is_loop_stopped(*self.owner).await
    }
}

pub struct ReturnAccess<'a> {
    owner: &'a Uuid,
    cxs: &'a ExecutionContexts,
}

impl ReturnAccess<'_> {
    pub async fn open_cx(&self, uuid: &Uuid) -> Result<(), E> {
        self.cxs.open_return_cx(*self.owner, uuid).await
    }

    pub async fn close_cx(&self) -> Result<(), E> {
        self.cxs.close_return_cx(*self.owner).await
    }

    pub async fn set_vl(&self, vl: RtValue) -> Result<(), E> {
        self.cxs.set_return_vl(*self.owner, vl).await
    }

    pub async fn withdraw_vl(&self, uuid: &Uuid) -> Result<Option<RtValue>, E> {
        self.cxs.withdraw_return_vl(*self.owner, uuid).await
    }
}

pub struct CwdAccess<'a> {
    owner: &'a Uuid,
    cxs: &'a ExecutionContexts,
}

impl CwdAccess<'_> {
    pub async fn set(&self, path: PathBuf) -> Result<(), E> {
        self.cxs.set_cwd(*self.owner, path).await
    }

    pub async fn get(&self) -> Result<PathBuf, E> {
        self.cxs.get_cwd(*self.owner).await
    }

    pub async fn root(&self) -> Result<PathBuf, E> {
        self.cxs.get_root_cwd(*self.owner).await
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    owner: Uuid,
    cxs: ExecutionContexts,
}

impl ExecutionContext {
    pub fn new(owner: Uuid, cxs: ExecutionContexts) -> Self {
        Self { owner, cxs }
    }
    pub fn loops(&self) -> LoopAccess<'_> {
        LoopAccess {
            owner: &self.owner,
            cxs: &self.cxs,
        }
    }
    pub fn returns(&self) -> ReturnAccess<'_> {
        ReturnAccess {
            owner: &self.owner,
            cxs: &self.cxs,
        }
    }
    pub fn scopes(&self) -> ScopeAccess<'_> {
        ScopeAccess {
            owner: &self.owner,
            cxs: &self.cxs,
        }
    }
    pub fn values(&self) -> ValueAccess<'_> {
        ValueAccess {
            owner: &self.owner,
            cxs: &self.cxs,
        }
    }
    pub fn cwd(&self) -> CwdAccess<'_> {
        CwdAccess {
            owner: &self.owner,
            cxs: &self.cxs,
        }
    }
    pub async fn storage(&self) -> Result<Storage, E> {
        Ok(Storage::create(
            self.cwd()
                .root()
                .await?
                .join(SIBS_FOLDER)
                .join(STORAGE_FOLDER),
        )?)
    }
    pub async fn child(&self, owner: Uuid) -> Result<ExecutionContext, E> {
        Ok(Self::new(owner, self.cxs.clone()))
    }

    pub async fn close(&self) -> Result<(), E> {
        self.cxs.close_cx(self.owner).await
    }
}
