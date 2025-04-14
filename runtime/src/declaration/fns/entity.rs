use crate::*;

#[derive(Debug)]
pub enum FnEntity<'a> {
    EFn(&'a EmbeddedFnEntity),
    UFn(&'a UserFnEntity),
    CFn(&'a ClosureFnEntity),
}

impl FnEntity<'_> {
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
        cx: Context,
        args: Vec<FnArgValue>,
        fns: &Fns,
        caller: &SrcLink,
    ) -> Result<RtValue, LinkedErr<E>> {
        match self {
            Self::UFn(en) => en.execute(rt, cx, args, fns, caller).await,
            Self::EFn(en) => en.execute(rt, cx, args, fns, caller).await,
            Self::CFn(en) => en.execute(rt, cx, args, fns, caller).await,
        }
    }

    pub fn args_compatible(args: &[&Ty], incomes: &[&Ty]) -> bool {
        let mut incomes = incomes.iter();
        let mut repeated = false;
        for arg_ty in args.iter() {
            if repeated {
                return false;
            }
            match arg_ty {
                Ty::Determined(..) | Ty::OneOf(..) | Ty::Variants(..) | Ty::Optional(..) => {
                    let Some(in_ty) = incomes.next() else {
                        return false;
                    };
                    let Some(in_ty) = in_ty.determined() else {
                        return false;
                    };
                    if !match arg_ty {
                        Ty::Determined(arg_ty) | Ty::Variants(arg_ty) | Ty::Optional(arg_ty) => {
                            arg_ty.compatible(in_ty)
                        }
                        Ty::OneOf(arg_tys) => arg_tys.iter().any(|arg_ty| arg_ty.compatible(in_ty)),
                        _ => true,
                    } {
                        return false;
                    }
                }
                Ty::Repeated(arg_ty) => {
                    repeated = true;
                    for in_ty in incomes.by_ref() {
                        let Some(in_ty) = in_ty.determined() else {
                            return false;
                        };
                        if !arg_ty.compatible(in_ty) {
                            return false;
                        }
                    }
                }
                Ty::Undefined | Ty::Indeterminate => {
                    return false;
                }
            }
        }
        true
    }
}
