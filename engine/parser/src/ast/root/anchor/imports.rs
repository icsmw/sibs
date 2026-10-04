use crate::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileKind {
    Working,
    Globals,
    Envs,
}

impl FileKind {
    fn name(self) -> &'static str {
        match self {
            Self::Working => "working",
            Self::Globals => "globals",
            Self::Envs => "envs",
        }
    }
}

/// Check the import graph owned by this anchor, without retaining another AST registry.
pub(super) fn validate(anchor: &Anchor) -> Result<(), LinkedErr<E>> {
    fn visit(node: &LinkedNode, files: &mut HashMap<Uuid, FileKind>) -> Result<(), LinkedErr<E>> {
        let imported = match node.get_node() {
            Node::Root(Root::GlobalsImport(n)) => {
                Some((*n.root.uuid(), FileKind::Globals, &n.node))
            }
            Node::Root(Root::EnvsImport(n)) => Some((*n.root.uuid(), FileKind::Envs, &n.node)),
            Node::Declaration(Declaration::IncludeDeclaration(n)) => {
                Some((*n.root.uuid(), FileKind::Working, &n.node))
            }
            Node::Declaration(Declaration::ModuleDeclaration(n)) => {
                Some((n.source, FileKind::Working, &n.node))
            }
            _ => None,
        };
        if let Some((source, kind, path)) = imported {
            if let Some(previous) = files.insert(source, kind) {
                if previous != kind {
                    return Err(E::MissedExpectation(
                        path.to_string(),
                        format!("{} file, not {}", previous.name(), kind.name()),
                    )
                    .link(path.as_ref()));
                }
            }
        }
        for child in node.childs() {
            visit(child, files)?;
        }
        Ok(())
    }
    let mut files = HashMap::from([(anchor.uuid, FileKind::Working)]);
    for node in &anchor.nodes {
        visit(node, &mut files)?;
    }
    Ok(())
}
