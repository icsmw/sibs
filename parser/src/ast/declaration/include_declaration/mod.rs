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
        let node = LinkedNode::try_oneof(
            parser,
            &[NodeReadTarget::Value(&[ValueId::PrimitiveString])],
        )?
        .ok_or_else(|| E::MissedModulePath.link_with_token(&sig))?;
        Ok(Some(IncludeDeclaration {
            sig,
            from,
            node: Box::new(node),
            uuid: Uuid::new_v4(),
        }))
    }
}
