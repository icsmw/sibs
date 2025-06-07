#[cfg(test)]
mod tests;

mod search;
mod suggestions;

pub use search::*;
pub use suggestions::*;

use crate::*;
use runtime::{DeterminedTy, TyCompatibility, TypeEntity};
use suggestions::{funcs::FnTypeKind, *};

#[derive(Debug)]
enum Filter {
    Variables(Option<Ty>),
    FunctionArgument(Option<Ty>),
    FunctionCall(Option<Ty>),
    All(Option<Ty>),
}

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
    token: Token,
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
        let token = current.token.clone();
        drop(current);
        debug!("Prev token: {}", token.id());
        let tree = self.locator.get_ownership_tree(token.pos.from.abs);
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
        self.locator.drop();
        if let Some(ownership) = ownership {
            Ok(Some(LocationData {
                ownership,
                blocks,
                mods,
                token,
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
        let Some(info) = self.get_location_data()? else {
            return Ok(None);
        };
        debug!("location data has been gotten");
        let Some(scope) = self.scx.tys.get_scope(&*info.get_scx_uuid()) else {
            return Ok(None);
        };
        debug!("scope has been detected");
        let cursor = info.token.id();
        // shift position to the right before cursor
        let before = self.locator.prev();
        if let Some(before) = &before {
            if matches!(before.token.kind, Kind::Keyword(Keyword::Let)) {
                return Ok(None);
            }
        }
        drop(before);

        let filter = loop {
            let Some(prev) = self.locator.prev() else {
                break None;
            };
            let prev_kind = prev.token.kind.clone();
            debug!("cursor token: {cursor}; prev token: {}", prev_kind.id());
            drop(prev);
            match &cursor {
                KindId::Dot => {
                    let Some(before) = self.locator.prev_node() else {
                        return Ok(None);
                    };
                    let ty = find_ty_by_node(&before.node, scope, self.scx);
                    break Some(Filter::FunctionArgument(ty));
                }
                _ => {}
            }
            match prev_kind {
                Kind::Dot => {
                    let ty = find_ty(&mut self.locator, scope, self.scx);
                    match cursor {
                        KindId::Identifier | KindId::Dot => {
                            break Some(Filter::FunctionArgument(ty));
                        }
                        _ => {}
                    }
                }
                Kind::Number(..) => match cursor {
                    KindId::Dot => {
                        break Some(Filter::FunctionArgument(Some(Ty::Determined(
                            DeterminedTy::Num,
                        ))));
                    }
                    _ => {
                        break None;
                    }
                },
                Kind::String(..) | Kind::SingleQuote => match cursor {
                    KindId::Dot => {
                        break Some(Filter::FunctionArgument(Some(Ty::Determined(
                            DeterminedTy::Str,
                        ))));
                    }
                    _ => {
                        break None;
                    }
                },
                Kind::Keyword(Keyword::Let) => {
                    break None;
                }
                Kind::Keyword(Keyword::True) | Kind::Keyword(Keyword::False) => match cursor {
                    KindId::Dot => {
                        break Some(Filter::FunctionArgument(Some(Ty::Determined(
                            DeterminedTy::Bool,
                        ))));
                    }
                    _ => {
                        break None;
                    }
                },
                Kind::Keyword(Keyword::If) => match cursor {
                    KindId::Identifier => {
                        break Some(Filter::All(None));
                    }
                    _ => {}
                },
                Kind::LeftParen => {}
                Kind::Whitespace(..) => {}
                Kind::LeftBrace => {}
                Kind::Colon => {}
                Kind::Comma => {}
                Kind::Semicolon => {
                    break Some(Filter::All(None));
                }
                Kind::Equals => {
                    let ty = find_ty(&mut self.locator, scope, self.scx);
                    break match cursor {
                        KindId::Identifier => Some(Filter::All(ty)),
                        _ => Some(Filter::All(None)),
                    };
                }
                Kind::Plus
                | Kind::Minus
                | Kind::Star
                | Kind::Slash
                | Kind::Less
                | Kind::Greater
                | Kind::LessEqual
                | Kind::GreaterEqual => {
                    break Some(Filter::All(Some(Ty::Determined(DeterminedTy::Num))));
                }
                _ => {}
            };
        };
        let Some(filter) = filter else {
            return Ok(None);
        };
        let suggestion = match &filter {
            Filter::Variables(ty) => Some(vars::collect(
                &scope,
                &info.blocks,
                &self.fragment,
                ty.as_ref(),
                self.from,
            )),
            Filter::FunctionArgument(ty) | Filter::FunctionCall(ty) => Some(funcs::collect(
                &self.scx.fns,
                &info.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
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
                    vars::collect(&scope, &info.blocks, &self.fragment, None, self.from);
                suggestions.extend(funcs::collect(
                    &self.scx.fns,
                    &info.mods.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
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

fn find_ty<'a>(
    locator: &mut LocationIterator<'a>,
    scope: &TyScope,
    scx: &SemanticCx,
) -> Option<Ty> {
    find_ty_by_node(locator.prev()?.node?, scope, scx)
}

fn find_ty_by_node<'a>(node: &LinkedNode, scope: &TyScope, scx: &SemanticCx) -> Option<Ty> {
    match node.get_node() {
        Node::Expression(Expression::Variable(node)) => {
            scx.find_linked_ty(&node.uuid).or_else(|| {
                scope
                    .lookup(&node.ident)
                    .map(|entity| entity.ty())
                    .flatten()
                    .cloned()
            })
        }
        Node::Declaration(Declaration::VariableName(node)) => scope
            .lookup(&node.ident)
            .map(|entity| entity.ty())
            .flatten()
            .cloned(),
        Node::Declaration(Declaration::VariableType(node)) => scx.table.get(&node.uuid),
        // Node::Declaration(Declaration::VariableVariants(..))
        Node::Declaration(Declaration::VariableTypeDeclaration(node)) => node
            .types
            .first()
            .map(|n| find_ty_by_node(n, scope, scx))
            .flatten(),
        _ => None,
    }
}
