#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use crate::*;
use std::sync::Arc;

impl Interest for ModuleDeclaration {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Mod))
    }
}

impl GetFilename for ModuleDeclaration {
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

impl ReadNode<ModuleDeclaration> for ModuleDeclaration {
    fn read(parser: &Parser) -> Result<Option<ModuleDeclaration>, LinkedErr<E>> {
        let Some(sig) = parser.token() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let Some(from) = parser.token() else {
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
        let (body, name) = {
            let Node::Value(Value::PrimitiveString(filename)) = filename_node.get_node() else {
                return Err(E::UnexpectedType(
                    ValueId::PrimitiveString.to_string(),
                    filename_node.get_node().id().to_string(),
                )
                .link(&filename_node));
            };
            let filepath = PathBuf::from(&filename.inner);
            let Some(name) = filepath
                .file_stem()
                .map(|name| name.to_string_lossy().to_string())
            else {
                return Err(E::FailGetModuleName(filename.inner.clone()).link(&filename_node));
            };
            (read_body(parser, filename)?, name)
        };
        #[cfg(test)]
        let (body, name) = (
            Arc::new(ModuleBody {
                nodes: Vec::new(),
                source: Uuid::new_v4(),
            }),
            String::from("test"),
        );
        Ok(Some(ModuleDeclaration {
            sig: sig.clone(),
            from: from.clone(),
            node: Box::new(filename_node),
            uuid: Uuid::new_v4(),
            name,
            body,
        }))
    }
}

fn read_body(parser: &Parser, filename: &PrimitiveString) -> Result<Arc<ModuleBody>, LinkedErr<E>> {
    match parser
        .prepare_module(&filename.inner)
        .map_err(|err| LinkedErr::from(err, filename))?
    {
        ModuleLoad::Unparsed {
            parser: inner,
            path,
        } => {
            let nodes = read_nodes(&inner)?;
            inner
                .flush()
                .map_err(|err| LinkedErr::from(err, filename))?;
            let body = Arc::new(ModuleBody {
                source: inner.src(),
                nodes,
            });
            parser.modules.borrow_mut().insert(path, body.clone());
            Ok(body)
        }
        ModuleLoad::Cached(body) => Ok(body),
    }
}

fn read_nodes(inner: &Parser) -> Result<Vec<LinkedNode>, LinkedErr<E>> {
    let mut nodes = Vec::new();
    loop {
        while inner.is_next(KindId::Semicolon) {
            let _ = inner.token();
        }
        let Some(node) = LinkedNode::try_oneof(
            inner,
            &[
                NodeTarget::Declaration(&[
                    DeclarationId::FunctionDeclaration,
                    DeclarationId::ModuleDeclaration,
                ]),
                NodeTarget::Root(&[RootId::Module, RootId::GlobalsImport, RootId::EnvsImport]),
            ],
        )?
        else {
            break;
        };
        nodes.push(node);
    }
    if !inner.is_done() {
        Err(E::UnrecognizedCode(inner.to_string()).link_until_end(inner))
    } else {
        Ok(nodes)
    }
}
