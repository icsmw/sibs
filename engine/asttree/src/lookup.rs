use crate::*;

#[derive(Debug)]
pub struct FoundNode<'a> {
    pub owner: Uuid,
    pub node: &'a LinkedNode,
}

pub trait Lookup<'a> {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>>;
}

pub trait FindMutByUuid {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode>;
}

impl FindMutByUuid for Vec<LinkedNode> {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        for node in self.iter_mut() {
            if node.uuid() == uuid {
                return Some(node);
            }
            let nested = node.node.find_mut_by_uuid(uuid);
            if nested.is_some() {
                return nested;
            }
        }
        None
    }
}

impl FindMutByUuid for LinkedNode {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        if self.uuid() == uuid {
            Some(self)
        } else {
            self.node.find_mut_by_uuid(uuid)
        }
    }
}

impl FindMutByUuid for Option<Box<LinkedNode>> {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        if let Some(node) = self.as_mut() {
            node.find_mut_by_uuid(uuid)
        } else {
            None
        }
    }
}

pub trait LookupInner<'a> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>>;
}

impl<'a> LookupInner<'a> for Vec<&'a LinkedNode> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        let nested: Vec<FoundNode<'a>> = self.iter().flat_map(|n| n.lookup(trgs)).collect();
        self.into_filtered_nodes(trgs)
            .into_iter()
            .map(|node| FoundNode { owner, node })
            .chain(nested)
            .collect()
    }
}

impl<'a> LookupInner<'a> for &'a LinkedNode {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.into_filtered_nodes(trgs)
            .into_iter()
            .map(|node| FoundNode { owner, node })
            .chain(self.lookup(trgs))
            .collect()
    }
}

impl<'a> LookupInner<'a> for Option<&'a Box<LinkedNode>> {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.map(|n| {
            n.lookup_inner(owner, trgs)
                .into_iter()
                .chain(n.lookup(trgs))
                .collect()
        })
        .unwrap_or_default()
    }
}
