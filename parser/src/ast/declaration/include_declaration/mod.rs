#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for IncludeDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Include))
    }
}

impl GetFilename for IncludeDeclaration {
    fn get_filename(&self) -> Result<PathBuf, E> {
        let Node::Value(Value::PrimitiveString(val)) = &self.node.node else {
            return Err(E::UnexpectedType(
                ValueId::PrimitiveString.to_string(),
                self.node.node.id().to_string(),
            ));
        };
        Ok(PathBuf::from(&val.inner))
    }
}

impl ReadNode<IncludeDeclaration> for IncludeDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<IncludeDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Include)) {
            return Ok(None);
        }
        let Some(from) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::Identifier(vl) = &from.kind else {
            return Ok(None);
        };
        if vl != "from" {
            return Ok(None);
        }
        let filename_node =
            LinkedNode::try_oneof(parser, &[NodeTarget::Value(&[ValueId::PrimitiveString])])?
                .ok_or_else(|| E::MissedModulePath.link_with_token(&sig))?;
        #[cfg(not(test))]
        let root = {
            let Node::Value(Value::PrimitiveString(filename)) = &filename_node.node else {
                return Err(E::UnexpectedType(
                    ValueId::PrimitiveString.to_string(),
                    filename_node.node.id().to_string(),
                )
                .link(&filename_node));
            };
            let mut inner = parser
                .from_file(&filename.inner)
                .map_err(|e| e.link(&filename_node))?;
            let Some(root) =
                LinkedNode::try_oneof(&mut inner, &[NodeTarget::Root(&[RootId::Anchor])])?
            else {
                return Err(E::FailToFindNode(RootId::Anchor.to_string()).link(&filename_node));
            };
            if !inner.is_done() {
                return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
            }
            root
        };
        #[cfg(test)]
        let root = {
            LinkedNode::from_node(Node::Root(Root::Anchor(Anchor {
                nodes: Vec::new(),
                uuid: Uuid::new_v4(),
            })))
        };
        Ok(Some(IncludeDeclaration {
            sig,
            from,
            node: Box::new(filename_node),
            root: Box::new(root),
            uuid: Uuid::new_v4(),
        }))
    }
}
