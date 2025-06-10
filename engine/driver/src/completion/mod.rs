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

#[derive(Debug)]
enum Ownership {
    Task(Uuid),
    /// * `Uuid` - Uuid of function declaration node
    /// * `Option<Uuid>` - Uuid of task, if function is declared in context of task
    Function(Uuid, Option<Uuid>),
}

#[derive(Debug)]
struct LocationData {
    ownership: Ownership,
    blocks: Vec<Uuid>,
    /// Location in modules
    mods: Vec<String>,
    cursor: Token,
    before_token: Option<Token>,
    before_node: Option<Uuid>,
}

impl LocationData {
    pub fn get_scx_uuid(&self) -> &Uuid {
        match &self.ownership {
            Ownership::Task(uuid) => uuid,
            Ownership::Function(_, Some(uuid)) => uuid,
            Ownership::Function(uuid, None) => uuid,
        }
    }
}

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
    fn get_location_data(&mut self) -> Result<Option<LocationData>, E> {
        let Some(current) = self.locator.prev_token() else {
            self.locator.drop();
            return Ok(None);
        };
        let cursor = current.token.clone();
        drop(current);
        debug!("Prev token: {}", cursor.id());
        let tree = self.locator.get_ownership_tree(cursor.pos.from.abs);
        let mut blocks = Vec::new();
        let mut mods = Vec::new();
        let mut ownership = None;
        for node in tree.iter().rev().into_iter() {
            match node.get_node() {
                Node::Statement(Statement::Block(..)) => {
                    blocks.push(*node.uuid());
                }
                Node::Declaration(Declaration::FunctionDeclaration(..)) => {
                    if ownership.is_none() {
                        ownership = Some(Ownership::Function(*node.uuid(), None));
                    } else {
                        return Err(E::TaskInsideFuncDeclaration(*node.uuid()));
                    }
                }
                Node::Root(Root::Module(module)) => {
                    if let Some(name) = module.get_name() {
                        mods.insert(0, name.to_owned());
                    }
                }
                Node::Root(Root::Task(..)) => {
                    if let Some(Ownership::Function(uuid, task)) = ownership {
                        if task.is_some() {
                            return Err(E::NestedTasks(*node.uuid()));
                        }
                        ownership = Some(Ownership::Function(uuid, Some(*node.uuid())));
                    } else {
                        ownership = Some(Ownership::Task(*node.uuid()));
                    }
                    break;
                }
                _ => {}
            }
        }
        let restore = self.locator.pin();
        let before_token = self.locator.prev().map(|prev| prev.token.clone());
        restore(&mut self.locator);
        let before_node = self
            .locator
            .prev_node()
            .map(|prev| prev.node.uuid().to_owned());
        self.locator.drop();
        if let Some(ownership) = ownership {
            Ok(Some(LocationData {
                ownership,
                blocks,
                mods,
                cursor,
                before_token,
                before_node,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn suggest(&mut self) -> Result<Option<Vec<CompletionSuggestion>>, E> {
        debug!(
            "looking for suggestions based on fragment: \"{}\"",
            self.fragment
        );
        let Some(loc) = self.get_location_data()? else {
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
        let Some(before_node) = self.locator.find(&before_node) else {
            warn!("Fail to find node {before_node}");
            return Ok(None);
        };
        let Some(filter) = get_filter(&loc, &before_token, before_node, &self.scx, &ty_scope)
        else {
            return Ok(None);
        };
        let suggestion = match &filter {
            Filter::Variables(ty) => Some(vars::collect(
                &ty_scope,
                &loc.blocks,
                &self.fragment,
                ty.as_ref(),
                self.from,
            )),
            Filter::FunctionArgument(ty) | Filter::FunctionCall(ty) => Some(funcs::collect(
                &self.scx.fns,
                &loc.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                if self.fragment.trim() == "." {
                    ""
                } else {
                    &self.fragment
                },
                if matches!(filter, Filter::FunctionArgument(..)) {
                    FnTypeKind::FirstArg(ty.as_ref())
                } else {
                    FnTypeKind::Returns(ty.as_ref())
                },
            )),
            Filter::All(ty) => {
                let mut suggestions =
                    vars::collect(&ty_scope, &loc.blocks, &self.fragment, None, self.from);
                suggestions.extend(funcs::collect(
                    &self.scx.fns,
                    &loc.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                    &self.fragment,
                    FnTypeKind::Returns(ty.as_ref()),
                ));
                if suggestions.is_empty() {
                    None
                } else {
                    Some(suggestions)
                }
            }
        };
        Ok(suggestion)
    }
}
