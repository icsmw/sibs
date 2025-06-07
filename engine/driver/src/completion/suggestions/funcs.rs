use runtime::{EmbeddedFnEntity, UserFnEntity};

use crate::completion::*;
use crate::*;

#[derive(Debug)]
pub enum FnTypeKind<'a> {
    FirstArg(Option<&'a Ty>),
    Returns(Option<&'a Ty>),
}

impl<'a> FnTypeKind<'a> {
    pub fn filter_ufns(&self, fn_entity: &UserFnEntity) -> bool {
        match self {
            Self::FirstArg(ty) => ty
                .map(|ty| {
                    fn_entity
                        .args
                        .first()
                        .map(|arg| &arg.ty == ty)
                        .unwrap_or_default()
                })
                .unwrap_or(true),
            Self::Returns(ty) => ty.map(|ty| &fn_entity.result == ty).unwrap_or(true),
        }
    }
    pub fn filter_efns(&self, fn_entity: &EmbeddedFnEntity) -> bool {
        match self {
            Self::FirstArg(ty) => ty
                .map(|ty| {
                    fn_entity
                        .args
                        .first()
                        .map(|arg| arg.compatible(ty))
                        .unwrap_or_default()
                })
                .unwrap_or(false),
            Self::Returns(ty) => ty
                .map(|ty| {
                    if let Ty::Determined(ty) = ty {
                        fn_entity.result.compatible(ty)
                    } else {
                        false
                    }
                })
                .unwrap_or(true),
        }
    }
}

pub fn collect(
    fns: &Fns,
    mods: &[&str],
    fragment: &str,
    ty: FnTypeKind,
) -> Vec<CompletionSuggestion> {
    debug!("try to find function for \"{fragment}\", {ty:?}");
    let mut collected: Vec<CompletionSuggestion> = fns
        .ufns
        .collect_by_path(mods)
        .into_iter()
        .filter_map(|(name, fn_entity)| {
            if !ty.filter_ufns(fn_entity) {
                return None;
            }
            if fragment.trim().is_empty() {
                Some(CompletionSuggestion {
                    target: CompletionMatch::Function(
                        name.to_string(),
                        None,
                        fn_entity.args.first().map(|arg| arg.ty.clone()),
                        Some(fn_entity.result.clone()),
                    ),
                    score: search::MAX_SCORE,
                })
            } else if let Some(ranked) = search::rank_match(&name, fragment) {
                Some(CompletionSuggestion {
                    target: CompletionMatch::Function(
                        name.to_string(),
                        None,
                        fn_entity.args.first().map(|arg| arg.ty.clone()),
                        Some(fn_entity.result.clone()),
                    ),
                    score: ranked.score,
                })
            } else {
                None
            }
        })
        .collect();
    collected.extend(
        fns.efns
            .collect()
            .into_iter()
            .filter_map(|(name, fn_entity)| {
                if !ty.filter_efns(fn_entity) {
                    return None;
                }
                if fragment.trim().is_empty() {
                    Some(CompletionSuggestion {
                        target: CompletionMatch::Function(
                            name.to_string(),
                            Some(fn_entity.docs.clone()),
                            fn_entity.args.first().cloned(),
                            Some(Ty::Determined(fn_entity.result.clone())),
                        ),
                        score: search::MAX_SCORE,
                    })
                } else if let Some(ranked) = search::rank_match(&name, fragment) {
                    Some(CompletionSuggestion {
                        target: CompletionMatch::Function(
                            name.to_string(),
                            Some(fn_entity.docs.clone()),
                            fn_entity.args.first().cloned(),
                            Some(Ty::Determined(fn_entity.result.clone())),
                        ),
                        score: ranked.score,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<CompletionSuggestion>>(),
    );
    collected
}
