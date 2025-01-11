use crate::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsageCx {
    DeclaredArgFn,
    EmbeddedArgFn,
    ClosureArg,
    TaskArg,
}

pub trait TyCompatibility {
    fn ty_compatibility(&self) -> Vec<UsageCx>;
    fn is_ty_compatible(&self, cx: &UsageCx) -> bool {
        self.ty_compatibility().contains(cx)
    }
}

impl TyCompatibility for Ty {
    fn ty_compatibility(&self) -> Vec<UsageCx> {
        match self {
            Self::Indeterminate | Self::Undefined => Vec::new(),
            Self::Determined(ty) | Self::Variants(ty) | Self::Optional(ty) | Self::Repeated(ty) => {
                ty.ty_compatibility()
            }
            Self::OneOf(tys) => {
                if tys.is_empty() {
                    return Vec::new();
                }
                let mut cxs = Vec::new();
                vec![
                    UsageCx::DeclaredArgFn,
                    UsageCx::TaskArg,
                    UsageCx::ClosureArg,
                    UsageCx::EmbeddedArgFn,
                ]
                .into_iter()
                .for_each(|cx| {
                    if tys.iter().filter(|ty| ty.is_ty_compatible(&cx)).count() == tys.len() {
                        cxs.push(cx);
                    }
                });
                cxs
            }
        }
    }
}

impl TyCompatibility for DeterminedTy {
    fn ty_compatibility(&self) -> Vec<UsageCx> {
        match self {
            Self::Void => Vec::new(),
            Self::Recursion(..) => vec![UsageCx::DeclaredArgFn, UsageCx::EmbeddedArgFn],
            Self::Range | Self::Num | Self::Bool | Self::PathBuf | Self::Str => vec![
                UsageCx::ClosureArg,
                UsageCx::DeclaredArgFn,
                UsageCx::EmbeddedArgFn,
                UsageCx::TaskArg,
            ],
            Self::Vec(ty) => {
                if let Some(ty) = ty {
                    ty.ty_compatibility()
                } else {
                    Vec::new()
                }
            }
            Self::ExecuteResult | Self::Error | Self::Closure(..) | Self::Any => vec![
                UsageCx::ClosureArg,
                UsageCx::DeclaredArgFn,
                UsageCx::EmbeddedArgFn,
            ],
        }
    }
}
