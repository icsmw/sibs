#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Module {
    fn intrested(token: &Token) -> bool {
        matches!(token.kind, Kind::Keyword(Keyword::Mod))
    }
}

impl ReadNode<Module> for Module {
    fn read(parser: &Parser) -> Result<Option<Module>, LinkedErr<E>> {
        let Some(sig) = parser.token() else {
            return Ok(None);
        };
        if !matches!(sig.kind, Kind::Keyword(Keyword::Mod)) {
            return Ok(None);
        }
        let Some(name) = parser.token() else {
            return Ok(None);
        };
        let Kind::Identifier(vl) = &name.kind else {
            return Ok(None);
        };
        if vl == "from" {
            // This is declaration
            return Ok(None);
        }
        let Some((mut inner, open, close)) =
            parser.between(KindId::LeftBrace, KindId::RightBrace)?
        else {
            return Err(E::MissedModuleBody.link_with_token(&name));
        };
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
                &mut inner,
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
            return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
        }
        Ok(Some(Module {
            name: name.clone(),
            sig: sig.clone(),
            open: open.clone(),
            close: close.clone(),
            nodes,
            uuid: Uuid::new_v4(),
        }))
    }
}
