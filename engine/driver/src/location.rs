use crate::*;

pub struct LocationIteratorMatch<'a> {
    pub node: Option<&'a LinkedNode>,
    pub token: &'a Token,
}

impl<'a> LocationIteratorMatch<'a> {
    pub fn new(token: &'a Token, node: Option<&'a LinkedNode>) -> Self {
        Self { node, token }
    }
}

impl fmt::Display for LocationIteratorMatch<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} [{}]:{}",
            self.token.pos.from,
            self.token.pos.to,
            self.token.kind.id().to_string(),
            self.node.map(|n| n.ident()).unwrap_or(String::from("None"))
        )
    }
}
pub struct LocationIterator<'a> {
    anchor: &'a Anchor,
    src: Uuid,
    pos: usize,
    parser: &'a Parser,
    fingerprint: Option<String>,
}

impl<'a> LocationIterator<'a> {
    pub fn new(anchor: &'a Anchor, src: Uuid, pos: usize, parser: &'a Parser) -> Self {
        Self {
            anchor,
            src,
            pos,
            parser,
            fingerprint: None,
        }
    }

    pub fn prev<'s>(&'s mut self) -> Option<LocationIteratorMatch<'s>> {
        let found = loop {
            if self.pos == 0 {
                break false;
            }
            self.pos -= 1;
            let Some(token) = self.parser.get_token_by_pos(self.pos) else {
                break false;
            };
            if self
                .fingerprint
                .as_ref()
                .map(|fp| &token.fingerprint() != fp)
                .unwrap_or(true)
            {
                self.pos = token.pos.from;
                break true;
            }
        };
        if !found {
            None
        } else {
            self.find()
        }
    }

    pub fn next<'s>(&'s mut self) -> Option<LocationIteratorMatch<'s>> {
        let len = self.parser.len();
        let found = loop {
            if self.pos >= len {
                break false;
            }
            self.pos += 1;
            let Some(token) = self.parser.get_token_by_pos(self.pos) else {
                break false;
            };
            if self
                .fingerprint
                .as_ref()
                .map(|fp| &token.fingerprint() != fp)
                .unwrap_or(true)
            {
                self.pos = token.pos.to;
                break true;
            }
        };
        if !found {
            None
        } else {
            self.find()
        }
    }

    pub fn find<'s>(&'s mut self) -> Option<LocationIteratorMatch<'s>> {
        let node = find_node(self.anchor.childs(), &self.src, self.pos);
        let Some(token) = self.parser.get_token_by_pos(self.pos) else {
            return None;
        };
        self.fingerprint = Some(token.fingerprint());
        Some(LocationIteratorMatch::new(token, node))
    }
}
