#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Anchor {
    fn intrested(token: &Token) -> bool {
        matches!(
            token.kind,
            Kind::Keyword(Keyword::Component)
                | Kind::Keyword(Keyword::Task)
                | Kind::Keyword(Keyword::Mod)
                | Kind::Keyword(Keyword::Include)
        )
    }
}

impl ReadNode<Anchor> for Anchor {
    fn read(parser: &mut Parser) -> Result<Option<Anchor>, LinkedErr<E>> {
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
                        DeclarationId::ModuleDeclaration,
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
        Ok(Some(Anchor {
            nodes,
            uuid: Uuid::new_v4(),
        }))
    }
}
