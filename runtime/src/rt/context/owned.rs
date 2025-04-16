use crate::*;

pub struct ContextValues<'a> {
    owner: &'a Uuid,
    rt: &'a RtContext,
}

impl ContextValues<'_> {
    pub async fn set_parent_vl(&self, vl: ParentValue) -> Result<(), E> {
        self.rt.set_parent_vl(*self.owner, vl).await
    }

    pub async fn withdraw_parent_vl(&self) -> Result<Option<ParentValue>, E> {
        self.rt.withdraw_parent_vl(*self.owner).await
    }

    pub async fn drop_parent_vl(&self) -> Result<(), E> {
        self.rt.drop_parent_vl(*self.owner).await
    }

    pub async fn insert<S: ToString>(&self, name: S, vl: RtValue) -> Result<(), E> {
        self.rt.insert(*self.owner, name, vl).await
    }

    pub async fn update<S: ToString>(&self, name: S, vl: RtValue) -> Result<(), E> {
        self.rt.update(*self.owner, name, vl).await
    }

    pub async fn lookup<S: ToString>(&self, name: S) -> Result<Option<Arc<RtValue>>, E> {
        self.rt.lookup(*self.owner, name).await
    }
}

pub struct ContextLocation<'a> {
    owner: &'a Uuid,
    rt: &'a RtContext,
}

impl ContextLocation<'_> {
    pub async fn open(&self, uuid: &Uuid) -> Result<(), E> {
        self.rt.open(*self.owner, uuid).await
    }

    pub async fn close(&self) -> Result<(), E> {
        self.rt.close(*self.owner).await
    }

    pub async fn enter(&self, uuid: &Uuid) -> Result<(), E> {
        self.rt.enter(*self.owner, uuid).await
    }

    pub async fn leave(&self) -> Result<(), E> {
        self.rt.leave(*self.owner).await
    }
}

pub struct ContextLoop<'a> {
    owner: &'a Uuid,
    rt: &'a RtContext,
}

impl ContextLoop<'_> {
    pub async fn open(&self, uuid: &Uuid) -> Result<(), E> {
        self.rt.open_loop(*self.owner, uuid).await
    }

    pub async fn close(&self) -> Result<(), E> {
        self.rt.close_loop(*self.owner).await
    }

    pub async fn set_break(&self) -> Result<(), E> {
        self.rt.set_break(*self.owner).await
    }

    pub async fn is_stopped(&self) -> Result<bool, E> {
        self.rt.is_loop_stopped(*self.owner).await
    }
}

pub struct ContextReturns<'a> {
    owner: &'a Uuid,
    rt: &'a RtContext,
}

impl ContextReturns<'_> {
    pub async fn open_cx(&self, uuid: &Uuid) -> Result<(), E> {
        self.rt.open_return_cx(*self.owner, uuid).await
    }

    pub async fn close_cx(&self) -> Result<(), E> {
        self.rt.close_return_cx(*self.owner).await
    }

    pub async fn set_vl(&self, vl: RtValue) -> Result<(), E> {
        self.rt.set_return_vl(*self.owner, vl).await
    }

    pub async fn withdraw_vl(&self, uuid: &Uuid) -> Result<Option<RtValue>, E> {
        self.rt.withdraw_return_vl(*self.owner, uuid).await
    }
}

pub struct ContextCwd<'a> {
    owner: &'a Uuid,
    rt: &'a RtContext,
}

impl ContextCwd<'_> {
    pub async fn set_cwd(&self, path: PathBuf) -> Result<(), E> {
        self.rt.set_cwd(*self.owner, path).await
    }

    pub async fn get_cwd(&self) -> Result<PathBuf, E> {
        self.rt.get_cwd(*self.owner).await
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    owner: Uuid,
    rt: RtContext,
    pub job: Job,
}

impl Context {
    pub fn new(owner: Uuid, rt: RtContext, job: Job) -> Self {
        Self { owner, rt, job }
    }
    pub fn loops(&self) -> ContextLoop<'_> {
        ContextLoop {
            owner: &self.owner,
            rt: &self.rt,
        }
    }
    pub fn returns(&self) -> ContextReturns<'_> {
        ContextReturns {
            owner: &self.owner,
            rt: &self.rt,
        }
    }
    pub fn location(&self) -> ContextLocation<'_> {
        ContextLocation {
            owner: &self.owner,
            rt: &self.rt,
        }
    }
    pub fn values(&self) -> ContextValues<'_> {
        ContextValues {
            owner: &self.owner,
            rt: &self.rt,
        }
    }
    pub fn cwd(&self) -> ContextCwd<'_> {
        ContextCwd {
            owner: &self.owner,
            rt: &self.rt,
        }
    }
    pub(crate) async fn child<S: ToString>(&self, owner: Uuid, alias: S) -> Result<Context, E> {
        Ok(self.rt.create(owner, self.job.child(owner, alias).await?))
    }
    pub(crate) async fn close(&self) -> Result<(), E> {
        self.rt.close_cx(self.owner).await?;
        self.job.close();
        Ok(())
    }
}
