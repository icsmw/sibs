use std::collections::HashMap;

use crate::*;

pub type KinshipMap<'a> = HashMap<Uuid, &'a LinkedNode>;
pub struct AnchorMap<'a> {
    anchor: &'a Anchor,
    kinship: KinshipMap<'a>,
}

impl<'a> AnchorMap<'a> {
    pub fn map(anchor: &'a Anchor) -> Self {
        fn mapping<'l>(
            parent: &'l LinkedNode,
            childs: Vec<&'l LinkedNode>,
            map: &mut KinshipMap<'l>,
        ) {
            childs.iter().for_each(|child| {
                map.insert(*child.uuid(), parent);
                mapping(*child, child.childs(), map);
            });
        }
        let mut kinship = HashMap::new();
        anchor.childs().iter().for_each(|parent| {
            mapping(parent, parent.childs(), &mut kinship);
        });
        Self { anchor, kinship }
    }
    pub fn find_parent<P: Fn(&LinkedNode) -> bool>(
        &self,
        uuid: &Uuid,
        predicate: P,
    ) -> Option<&'a LinkedNode> {
        let mut current = uuid;
        loop {
            if let Some(parent) = self.kinship.get(current) {
                if predicate(*parent) {
                    return Some(&parent);
                }
                current = parent.uuid();
            } else {
                break;
            }
        }
        None
    }
}
