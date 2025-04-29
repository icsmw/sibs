#[cfg(feature = "proptests")]
mod proptests;

use std::collections::HashMap;

use crate::*;

pub type TasksMetadata<'a> = Vec<(String, &'a Metadata)>;
pub type ComponentMetadata<'a> = (&'a Metadata, TasksMetadata<'a>);
pub type AnchorMetadata<'a> = HashMap<String, ComponentMetadata<'a>>;

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub ppm: Vec<LinkedNode>,
    pub meta: Vec<LinkedNode>,
    pub link: SrcLink,
}

impl Metadata {
    pub fn merge(&mut self, md: Metadata) {
        self.meta = md.meta;
        self.ppm = md.ppm;
    }
    pub fn meta_to_string(&self) -> String {
        if self.meta.is_empty() {
            String::new()
        } else {
            format!(
                "{}{}{}",
                Kind::LF,
                self.meta
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(&Kind::LF.to_string()),
                Kind::LF
            )
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.meta
            .iter()
            .filter_map(|n| {
                if let Node::Miscellaneous(Miscellaneous::Meta(mn)) = &n.node {
                    Some(mn.as_trimmed_string())
                } else {
                    None
                }
            })
            .collect()
    }

    #[cfg(feature = "proptests")]
    pub(crate) fn take_meta(&mut self, src: &mut LinkedNode) {
        self.meta.append(&mut src.md.meta);
    }
}

impl Diagnostic for Metadata {
    fn located(&self, src: &Uuid, pos: usize) -> bool {
        if !self.link.belongs(src) {
            false
        } else {
            self.get_position().is_in(pos)
        }
    }
    fn get_position(&self) -> Position {
        Position::new(self.link.exfrom(), self.link.exto())
    }
    fn childs(&self) -> Vec<&LinkedNode> {
        [
            self.ppm.iter().collect::<Vec<&LinkedNode>>(),
            self.meta.iter().collect::<Vec<&LinkedNode>>(),
        ]
        .concat()
    }
}
