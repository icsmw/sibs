#[cfg(feature = "proptests")]
mod proptests;

use crate::*;

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub ppm: Vec<LinkedNode>,
    pub meta: Vec<LinkedNode>,
    pub pos: Position,
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

    #[cfg(feature = "proptests")]
    pub(crate) fn take_meta(&mut self, src: &mut LinkedNode) {
        self.meta.append(&mut src.md.meta);
    }
}
