use asttree::{ExpressionId, LinkedNode, Metadata};

use crate::*;

impl ReadMetadata for Metadata {
    fn read_md_before(
        &mut self,
        parser: &Parser,
        includes: &[MiscellaneousId],
    ) -> Result<(), LinkedErr<E>> {
        self.meta.clear();
        loop {
            let reset = parser.pin();
            let mut found = None;
            for id in includes {
                found = match id {
                    MiscellaneousId::RootMeta => RootMeta::read(parser)?.map(|n| n.into()),
                    MiscellaneousId::Meta => Meta::read(parser)?.map(|n| n.into()),
                    MiscellaneousId::Comment => Comment::read(parser)?.map(|n| n.into()),
                };
                if found.is_some() {
                    break;
                }
                reset(parser);
            }
            match found {
                Some(node) => self.meta.push(LinkedNode::from_node(node)),
                None => break,
            }
        }
        Ok(())
    }
    fn read_md_after(&mut self, parser: &Parser) -> Result<(), LinkedErr<E>> {
        self.ppm = Vec::new();
        while let Some(node) = LinkedNode::try_oneof(
            parser,
            &[NodeTarget::Expression(&[
                ExpressionId::Accessor,
                ExpressionId::Call,
            ])],
        )? {
            self.ppm.push(node);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser(source: &str) -> Parser {
        let mut lx = lexer::Lexer::new(source, 0);
        Parser::unbound(lx.read().unwrap().tokens, &lx.uuid, source, false)
    }

    #[test]
    fn root_metadata_and_item_metadata_have_separate_owners() {
        for header in ["", "//! Script documentation\n// Header comment\n"] {
            let source = format!("{header}/// Component documentation\ncomponent comp() {{\n/// Task documentation\ntask run() {{ true; }}\n}};");
            let parser = parser(&source);
            let node = LinkedNode::try_read(&parser, NodeTarget::Root(&[RootId::Anchor]))
                .unwrap()
                .unwrap();
            assert!(parser.is_done());
            assert_eq!(
                node.get_md().lines(),
                if header.is_empty() {
                    vec![]
                } else {
                    vec!["Script documentation"]
                }
            );
            let anchor = node.extract::<Anchor>().unwrap();
            let component = anchor.get_component("comp").unwrap();
            assert_eq!(component.get_md().lines(), ["Component documentation"]);
            let tasks = component.extract::<Component>().unwrap().get_tasks_md();
            assert_eq!(tasks[0].1.lines(), ["Task documentation"]);
        }
    }

    #[test]
    fn root_metadata_in_the_wrong_position_is_rejected() {
        for source in [
            "/// Component documentation\n//! Misplaced root documentation\ncomponent comp() {};",
            "component comp() {\n//! Misplaced root documentation\ntask run() { true; }\n};",
            "component first() {};\n//! Misplaced root documentation\ncomponent second() {};",
        ] {
            assert!(
                LinkedNode::try_read(&parser(source), NodeTarget::Root(&[RootId::Anchor])).is_err(),
                "{source}"
            );
        }
    }

    #[test]
    fn metadata_reader_leaves_unaccepted_metadata_untouched() {
        let parser = parser("//! Root\n/// Item\ncomponent comp() {};");
        let mut md = Metadata::default();
        md.read_md_before(&parser, Anchor::md_includes()).unwrap();
        assert_eq!(md.lines(), ["Root"]);
        assert!(matches!(parser.next().unwrap().kind, Kind::Meta(_)));
        md.read_md_before(&parser, Component::md_includes())
            .unwrap();
        assert_eq!(md.lines(), ["Item"]);
        assert!(matches!(
            parser.next().unwrap().kind,
            Kind::Keyword(Keyword::Component)
        ));
    }

    #[test]
    fn root_metadata_is_readable_as_a_miscellaneous_node() {
        let parser = parser("//! Keep //! in the documentation");
        let node = LinkedNode::try_read(
            &parser,
            NodeTarget::Miscellaneous(&[MiscellaneousId::RootMeta]),
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            node.extract::<RootMeta>().unwrap().as_trimmed_string(),
            "Keep //! in the documentation"
        );
        assert!(node.get_md().meta.is_empty());
        assert!(parser.is_done());
    }
}
