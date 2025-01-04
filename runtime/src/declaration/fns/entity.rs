use crate::*;

#[derive(Debug)]
pub enum FnEntity<'a> {
    EFn(&'a EmbeddedFnEntity),
    UFn(&'a UserFnEntity),
    CFn(&'a ClosureFnEntity),
}

impl<'a> FnEntity<'a> {
    pub fn name(&self) -> &str {
        match self {
            Self::UFn(en) => &en.name,
            Self::EFn(en) => &en.fullname,
            Self::CFn(..) => "closure",
        }
    }
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::UFn(en) => &en.uuid,
            Self::EFn(en) => &en.uuid,
            Self::CFn(en) => &en.uuid,
        }
    }
    pub fn result_ty(&self) -> Ty {
        match self {
            Self::UFn(en) => en.result.clone(),
            Self::EFn(en) => Ty::Determined(en.result.clone()),
            Self::CFn(en) => en.result.clone(),
        }
    }
    pub fn args_tys(&self) -> Vec<&Ty> {
        match self {
            Self::UFn(en) => en.args.iter().map(|arg| &arg.ty).collect::<Vec<&Ty>>(),
            Self::EFn(en) => en.args.iter().collect::<Vec<&Ty>>(),
            Self::CFn(en) => en.args.iter().map(|arg| &arg.ty).collect::<Vec<&Ty>>(),
        }
    }
    pub async fn execute(
        &self,
        rt: Runtime,
        args: Vec<FnArgValue>,
        fns: &Fns,
    ) -> Result<RtValue, LinkedErr<E>> {
        match self {
            Self::UFn(en) => en.execute(rt, args, fns).await,
            Self::EFn(en) => en.execute(rt, args, fns).await,
            Self::CFn(en) => en.execute(rt, args, fns).await,
        }
    }
}
