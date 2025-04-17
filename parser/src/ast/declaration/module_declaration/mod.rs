#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for ModuleDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Mod))
    }
}

impl GetFilename for ModuleDeclaration {
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

impl ReadNode<ModuleDeclaration> for ModuleDeclaration {
    fn read(parser: &mut Parser) -> Result<Option<ModuleDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token().cloned() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Mod)) {
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
        let (nodes, name) = {
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
            let filepath = PathBuf::from(&filename.inner);
            let Some(name) = filepath
                .file_stem()
                .map(|name| name.to_string_lossy().to_string())
            else {
                return Err(E::FailGetModuleName(filename.inner.clone()).link(&filename_node));
            };
            (get_mod_inner(&mut inner)?, name)
        };
        #[cfg(test)]
        let (nodes, name) = { (Vec::new(), String::from("test")) };
        Ok(Some(ModuleDeclaration {
            sig,
            from,
            node: Box::new(filename_node),
            uuid: Uuid::new_v4(),
            name,
            nodes,
        }))
    }
}

#[cfg(not(test))]
fn get_mod_inner(inner: &mut Parser) -> Result<Vec<LinkedNode>, LinkedErr<E>> {
    let mut nodes = Vec::new();
    loop {
        'semicolons: loop {
            if inner.is_next(KindId::Semicolon) {
                let _ = inner.token();
            } else {
                break 'semicolons;
            }
        }
        let Some(node) = LinkedNode::try_oneof(
            inner,
            &[
                NodeTarget::Declaration(&[
                    DeclarationId::FunctionDeclaration,
                    DeclarationId::ModuleDeclaration,
                ]),
                NodeTarget::Root(&[RootId::Module]),
            ],
        )?
        else {
            break;
        };
        nodes.push(node);
    }
    if !inner.is_done() {
        Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner))
    } else {
        Ok(nodes)
    }
}
