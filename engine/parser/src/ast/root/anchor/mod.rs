mod imports;
#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl Interest for Anchor {
    fn intrested(token: &Token) -> bool {
        // The first declaration may start with its own documentation.
        matches!(
            token.kind,
            Kind::Meta(..)
                | Kind::Keyword(Keyword::Component)
                | Kind::Keyword(Keyword::Task)
                | Kind::Keyword(Keyword::Mod)
                | Kind::Keyword(Keyword::Include)
                | Kind::Keyword(Keyword::Globals)
                | Kind::Keyword(Keyword::Envs)
        )
    }
}

impl ReadNode<Anchor> for Anchor {
    fn read(parser: &Parser) -> Result<Option<Anchor>, LinkedErr<E>> {
        let mut nodes = Vec::new();
        loop {
            'semicolons: loop {
                if parser.is_next(KindId::Semicolon) {
                    let _ = parser.token();
                } else {
                    break 'semicolons;
                }
            }
            let Some(node) = LinkedNode::try_oneof(
                parser,
                &[
                    NodeTarget::Declaration(&[
                        DeclarationId::ModuleImport,
                        DeclarationId::GlobalsImport,
                        DeclarationId::EnvsImport,
                        DeclarationId::IncludeDeclaration,
                    ]),
                    NodeTarget::Root(&[RootId::Task, RootId::Component, RootId::Module]),
                ],
            )?
            else {
                break;
            };
            nodes.push(node);
        }
        if !parser.is_done() {
            return Err(E::UnrecognizedCode(parser.to_string()).link_until_end(parser));
        }
        let anchor = Anchor {
            nodes,
            uuid: parser.src(),
        };
        imports::validate(&anchor)?;
        Ok(Some(anchor))
    }
}
