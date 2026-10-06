#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for EnvsImport {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Envs))
    }
}

impl GetFilename for EnvsImport {
    fn get_filename(&self) -> Result<PathBuf, E> {
        let Node::Value(Value::PrimitiveString(val)) = self.node.get_node() else {
            return Err(E::UnexpectedType(
                ValueId::PrimitiveString.to_string(),
                self.node.get_node().id().to_string(),
            ));
        };
        Ok(PathBuf::from(&val.inner))
    }
}

impl ReadNode<EnvsImport> for EnvsImport {
    fn read(parser: &Parser) -> Result<Option<EnvsImport>, LinkedErr<E>> {
        let Some(sig) = parser.token() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Envs)) {
            return Ok(None);
        }
        let from = parser.token().ok_or_else(|| {
            E::MissedExpectation(sig.to_string(), "from".into()).link_with_token(&sig)
        })?;
        if !matches!(&from.kind, Kind::Identifier(vl) if vl == "from") {
            return Err(E::MissedExpectation(sig.to_string(), "from".into()).link_with_token(&from));
        }
        let filename_node =
            LinkedNode::try_oneof(parser, &[NodeTarget::Value(&[ValueId::PrimitiveString])])?
                .ok_or_else(|| E::MissedModulePath.link_with_token(&sig))?;
        let filename = PrimitiveString::extract(filename_node.get_node())
            .ok_or_else(|| E::MissedModulePath.link(&filename_node))?;
        let inner = parser
            .from_file(&filename.inner)
            .map_err(|err| err.link(&filename_node))?;
        // Read the module directly: an empty special file is valid too.
        let root = EnvsModule::read_as_linked(&inner)?.ok_or_else(|| {
            E::FailToFindNode(RootId::EnvsModule.to_string()).link(&filename_node)
        })?;
        inner.flush().map_err(|err| err.link(&filename_node))?;
        Ok(Some(EnvsImport {
            sig: sig.clone(),
            from: from.clone(),
            node: Box::new(filename_node),
            root: Box::new(root),
            uuid: Uuid::new_v4(),
        }))
    }
}
