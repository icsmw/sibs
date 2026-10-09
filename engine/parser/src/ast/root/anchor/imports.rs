use crate::*;
use std::collections::{HashMap, HashSet};

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

/// Check import kinds, visiting shared module bodies only once.
pub(super) fn validate(anchor: &Anchor) -> Result<(), LinkedErr<E>> {
    fn visit(
        node: &LinkedNode,
        files: &mut HashMap<Uuid, FileKind>,
        modules: &mut HashSet<Uuid>,
    ) -> Result<(), LinkedErr<E>> {
        let imported = match node.get_node() {
            Node::Declaration(Declaration::GlobalsImport(n)) => {
                Some((*n.root.uuid(), FileKind::Globals, &n.node))
            }
            Node::Declaration(Declaration::EnvsImport(n)) => {
                Some((*n.root.uuid(), FileKind::Envs, &n.node))
            }
            Node::Declaration(Declaration::IncludeDeclaration(n)) => {
                Some((*n.root.uuid(), FileKind::Working, &n.node))
            }
            Node::Declaration(Declaration::ModuleImport(n)) => {
                Some((n.body.source, FileKind::Working, &n.node))
            }
            _ => None,
        };
        if let Some((source, kind, path)) = imported
            && let Some(previous) = files.insert(source, kind)
            && previous != kind
        {
            return Err(E::MissedExpectation(
                path.to_string(),
                format!("{} file, not {}", previous.name(), kind.name()),
            )
            .link(path.as_ref()));
        }
        if let Some(module) = node.extract::<ModuleImport>()
            && !modules.insert(module.body.source)
        {
            return Ok(());
        }
        for child in node.childs() {
            visit(child, files, modules)?;
        }
        Ok(())
    }
    let mut files = HashMap::from([(anchor.uuid, FileKind::Working)]);
    let mut modules = HashSet::new();
    for node in &anchor.nodes {
        visit(node, &mut files, &mut modules)?;
    }
    Ok(())
}
