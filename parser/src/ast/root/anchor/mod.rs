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

#[cfg(any(not(test), all(test, feature = "io_tests")))]
fn read_file<N: GetFilename>(parser: &mut Parser, node: &N) -> Result<Parser, E> {
    let mut filename = node.get_filename()?;
    if filename.is_relative() {
        filename = parser.cwd.as_ref().ok_or(E::NoParentPath)?.join(filename);
    }
    if !filename.exists() {
        return Err(E::FileNotFound(filename.to_string_lossy().to_string()));
    }
    Parser::new(filename)
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
                    NodeReadTarget::Declaration(&[
                        DeclarationId::ModuleDeclaration,
                        DeclarationId::IncludeDeclaration,
                    ]),
                    NodeReadTarget::Root(&[RootId::Task, RootId::Component, RootId::Module]),
                ],
            )?
            else {
                break;
            };

            #[cfg(any(not(test), all(test, feature = "io_tests")))]
            {
                if let Node::Declaration(Declaration::IncludeDeclaration(incl)) = &node.node {
                    let mut inner = read_file(parser, incl).map_err(|e| e.link(&node))?;
                    let Some(node) = LinkedNode::try_oneof(
                        &mut inner,
                        &[NodeReadTarget::Root(&[RootId::Anchor])],
                    )?
                    else {
                        return Err(E::FailToFindNode(RootId::Anchor.to_string()).link(&node));
                    };
                    nodes.push(node);
                } else if let Node::Declaration(Declaration::ModuleDeclaration(incl)) = &node.node {
                    let mut inner = read_file(parser, incl).map_err(|e| e.link(&node))?;
                    let Some(node) = LinkedNode::try_oneof(
                        &mut inner,
                        &[NodeReadTarget::Root(&[RootId::Module])],
                    )?
                    else {
                        return Err(E::FailToFindNode(RootId::Module.to_string()).link(&node));
                    };
                    nodes.push(node);
                }
            }
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
