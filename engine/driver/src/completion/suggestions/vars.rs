use crate::completion::*;
use crate::*;

pub fn collect(
    scope: &TyScope,
    cxs: &[Uuid],
    fragment: &str,
    ty: Option<&Ty>,
) -> Vec<CompletionSuggestion> {
    let Some(variables) = scope.get_all_variables(cxs) else {
        return Vec::new();
    };
    let mut suggestions = Vec::new();
    for (name, ty_entity) in variables.iter() {
        let mut completion_match = if fragment.trim().is_empty() {
            Some(CompletionSuggestion {
                target: CompletionMatch::Variable(name.to_string()),
                score: search::MAX_SCORE,
            })
        } else if let Some(ranked) = search::rank_match(name, fragment) {
            Some(CompletionSuggestion {
                target: CompletionMatch::Variable(name.to_string()),
                score: ranked.score,
            })
        } else {
            None
        };
        if let (Some(completion_match), Some(target_ty)) = (completion_match.as_mut(), ty) {
            if let Some(ty) = ty_entity.ty() {
                if ty != target_ty {
                    completion_match.drop();
                }
            }
        }
        if let Some(completion_match) = completion_match {
            suggestions.push(completion_match);
        }
    }
    suggestions
}
