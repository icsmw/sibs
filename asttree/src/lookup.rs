use crate::*;

pub struct FoundNode<'a> {
    pub owner: Uuid,
    pub node: &'a LinkedNode,
}
pub trait Lookup<'a> {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>>;
}

pub trait LookupInner<'a> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>>;
}

impl<'a> LookupInner<'a> for Vec<&'a LinkedNode> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.into_filtered_nodes(trgs)
            .into_iter()
            .map(|node| FoundNode { owner, node })
            .collect()
    }
}

impl<'a> LookupInner<'a> for &'a LinkedNode {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.into_filtered_nodes(trgs)
            .into_iter()
            .map(|node| FoundNode { owner, node })
            .collect()
    }
}

impl<'a> LookupInner<'a> for Option<&'a Box<LinkedNode>> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.map(|n| n.lookup_inner(owner, trgs))
            .unwrap_or_default()
    }
}
