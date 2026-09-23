use crate::*;

pub trait Extract: Sized {
    fn extract(node: &Node) -> Option<&Self>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_matching_payload_by_reference() {
        let uuid = Uuid::new_v4();
        let node = LinkedNode::from_node(
            Anchor {
                nodes: Vec::new(),
                uuid,
            }
            .into(),
        );
        let anchor = node.extract::<Anchor>().unwrap();
        assert_eq!(anchor.uuid, uuid);
        let Node::Root(Root::Anchor(stored)) = node.get_node() else {
            unreachable!();
        };
        assert!(std::ptr::eq(anchor, stored));
        assert!(node.extract::<Root>().is_some());
        assert!(node.extract::<Component>().is_none());
        assert!(node.extract::<Block>().is_none());
    }
}
