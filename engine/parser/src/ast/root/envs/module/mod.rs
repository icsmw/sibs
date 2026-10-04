#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use crate::*;

impl Interest for EnvsModule {
    fn intrested(_token: &Token) -> bool {
        true
    }
}

impl ReadNode<EnvsModule> for EnvsModule {
    fn read_as_linked(parser: &Parser) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        Ok(Self::read(parser)?.map(|n| {
            let link = n.link();
            let mut node = LinkedNode::from_node(n.into());
            node.get_mut_md().link = link;
            node
        }))
    }

    fn read(parser: &Parser) -> Result<Option<EnvsModule>, LinkedErr<E>> {
        let mut nodes = Vec::new();
        while !parser.is_done() {
            if let Some(comment) = LinkedNode::try_read(
                parser,
                NodeTarget::Miscellaneous(&[MiscellaneousId::Comment]),
            )? {
                nodes.push(comment);
                continue;
            }
            let node = LinkedNode::try_read(
                parser,
                NodeTarget::Declaration(&[DeclarationId::EnvDeclaration]),
            )?
            .ok_or_else(|| E::MissedEnvRequirement.link_until_end(parser))?;
            if !node.get_md().ppm.is_empty() {
                return Err(E::UnrecognizedCode(node.to_string()).link(&node));
            }
            if !parser.is_next(KindId::Semicolon) {
                return Err(E::MissedSemicolon.link(&node));
            }
            let _ = parser.token();
            nodes.push(node);
        }
        Ok(Some(EnvsModule {
            nodes,
            uuid: parser.src(),
        }))
    }
}
