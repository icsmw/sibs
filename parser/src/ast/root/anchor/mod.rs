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
    parser.new_child(filename)
}

#[cfg(any(not(test), all(test, feature = "io_tests")))]
fn get_inner<N: GetFilename>(
    parser: &mut Parser,
    node: &N,
    linked: &LinkedNode,
    target: RootId,
) -> Result<LinkedNode, LinkedErr<E>> {
    let mut inner = read_file(parser, node).map_err(|e| e.link(linked))?;
    let Some(node) = LinkedNode::try_oneof(&mut inner, &[NodeTarget::Root(&[target.clone()])])?
    else {
        return Err(E::FailToFindNode(target.to_string()).link(linked));
    };
    Ok(node)
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

            #[cfg(any(not(test), all(test, feature = "io_tests")))]
            {
                if let Node::Declaration(Declaration::IncludeDeclaration(inn_node)) = &node.node {
                    nodes.push(get_inner(parser, inn_node, &node, RootId::Anchor)?);
                } else if let Node::Declaration(Declaration::ModuleDeclaration(inn_node)) =
                    &node.node
                {
                    nodes.push(get_inner(parser, inn_node, &node, RootId::Module)?);
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
