use crate::*;

#[derive(Debug)]
pub enum FnEntity<'a> {
    EFn(&'a EmbeddedFnEntity),
    UFn(&'a UserFnEntity),
}

impl<'a> FnEntity<'a> {
    pub fn name(&self) -> &str {
        match self {
            Self::UFn(en) => &en.name,
            Self::EFn(en) => &en.fullname,
        }
    }
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::UFn(en) => &en.uuid,
            Self::EFn(en) => &en.uuid,
        }
    }
    pub fn result_ty(&self) -> &DataType {
        match self {
            Self::UFn(en) => &en.result,
            Self::EFn(en) => &en.result,
        }
    }
    pub fn args_tys(&self) -> Vec<&DataType> {
        match self {
            Self::UFn(en) => en
                .args
                .iter()
                .map(|arg| &arg.ty)
                .collect::<Vec<&DataType>>(),
            Self::EFn(en) => en.args.iter().collect::<Vec<&DataType>>(),
        }
    }
    pub async fn execute(&self, rt: Runtime, args: Vec<RtValue>) -> Result<RtValue, LinkedErr<E>> {
        match self {
            Self::UFn(en) => en.execute(rt, args).await,
            Self::EFn(en) => en.execute(rt, args).await,
        }
    }
}
