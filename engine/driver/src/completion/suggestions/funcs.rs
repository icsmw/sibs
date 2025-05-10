use crate::completion::*;
use crate::*;

pub fn collect(
    fns: &Fns,
    mods: &[&str],
    fragment: &str,
    ty: Option<&Ty>,
) -> Vec<CompletionSuggestion> {
    let mut collected: Vec<CompletionSuggestion> = fns
        .ufns
        .collect_by_path(mods)
        .into_iter()
        .filter_map(|(name, ..)| {
            if fragment.trim().is_empty() {
                Some(CompletionSuggestion {
                    target: CompletionMatch::Function(name.to_string()),
                    score: search::MAX_SCORE,
                })
            } else if let Some(ranked) = search::rank_match(&name, fragment) {
                Some(CompletionSuggestion {
                    target: CompletionMatch::Function(name.to_string()),
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
            .filter_map(|(name, ..)| {
                if fragment.trim().is_empty() {
                    Some(CompletionSuggestion {
                        target: CompletionMatch::Function(name.to_string()),
                        score: search::MAX_SCORE,
                    })
                } else if let Some(ranked) = search::rank_match(&name, fragment) {
                    Some(CompletionSuggestion {
                        target: CompletionMatch::Function(name.to_string()),
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
