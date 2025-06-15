#[cfg(test)]
mod tests;

mod filter;
mod search;
mod suggestions;

use filter::*;
pub use search::*;
pub use suggestions::*;
use tracing::warn;

use crate::*;
use runtime::TypeEntity;
use suggestions::funcs::FnTypeKind;

pub struct Completion<'a> {
    locator: LocationIterator<'a>,
    scx: &'a SemanticCx,
    from: usize,
    fragment: String,
}

impl<'a> Completion<'a> {
    pub fn new(
        locator: LocationIterator<'a>,
        scx: &'a SemanticCx,
        fragment: String,
        from: usize,
    ) -> Self {
        Self {
            locator,
            scx,
            fragment,
            from,
        }
    }

    pub fn suggest(&mut self) -> Result<Option<CompletionResult>, E> {
        debug!(
            "looking for suggestions based on fragment: \"{}\"",
            self.fragment
        );
        let Some(loc) = Location::detect(&mut self.locator)? else {
            return Ok(None);
        };
        debug!("location data has been gotten");
        let Some(ty_scope) = self.scx.tys.get_scope(&*loc.get_scx_uuid()) else {
            return Ok(None);
        };
        debug!("scope has been detected");
        let (Some(before_token), Some(before_node)) = (&loc.before_token, &loc.before_node) else {
            return Ok(None);
        };
        debug!("token before: {:?}", before_token.id());
        let Some(before_node) = self.locator.find(&before_node) else {
            warn!("Fail to find node {before_node}");
            return Ok(None);
        };
        let Some(filter) = get_filter(&loc, &before_token, before_node, &self.scx, &ty_scope)
        else {
            debug!("no filters for pos: {}", self.from);
            return Ok(None);
        };
        let (fragment, replacement) = if let Cursor::Path(path, _, from, to) = &loc.cursor {
            (path, Some((*from, *to)))
        } else {
            (&self.fragment, None)
        };
        let suggestion = match &filter {
            Filter::Variables(ty) => Some(vars::collect(
                &ty_scope,
                &loc.blocks,
                fragment,
                ty.as_ref(),
                self.from,
            )),
            Filter::FunctionArgument(ty) | Filter::FunctionCall(ty) => Some(funcs::collect(
                &self.scx.fns,
                &loc.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                if fragment.trim() == "." {
                    ""
                } else {
                    &fragment
                },
                if matches!(filter, Filter::FunctionArgument(..)) {
                    FnTypeKind::FirstArg(ty.as_ref())
                } else {
                    FnTypeKind::Returns(ty.as_ref())
                },
            )),
            Filter::All(ty) => {
                let mut suggestions =
                    vars::collect(&ty_scope, &loc.blocks, fragment, None, self.from);
                suggestions.extend(funcs::collect(
                    &self.scx.fns,
                    &loc.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                    fragment,
                    FnTypeKind::Returns(ty.as_ref()),
                ));
                if suggestions.is_empty() {
                    None
                } else {
                    Some(suggestions)
                }
            }
        };
        Ok(suggestion.map(|suggestions| CompletionResult {
            suggestions,
            replacement,
        }))
    }
}
