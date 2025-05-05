use std::cell::Ref;

use crate::*;

pub struct TokenStep<'a> {
    pub node: Option<&'a LinkedNode>,
    pub token: Ref<'a, Token>,
}

impl<'a> TokenStep<'a> {
    pub fn new(token: Ref<'a, Token>, node: Option<&'a LinkedNode>) -> Self {
        Self { node, token }
    }
}

impl fmt::Display for TokenStep<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} [owner:{:?}][{}/{}]",
            self.token.pos.from,
            self.token.pos.to,
            self.token.owner,
            self.token.kind.id().to_string(),
            self.node
                .map(|n| format!("{}:{}", n.ident(), n.uuid()))
                .unwrap_or(String::from("None"))
        )
    }
}

pub struct NodeStep<'a> {
    pub node: &'a LinkedNode,
    pub tokens: Vec<Ref<'a, Token>>,
}

impl<'a> NodeStep<'a> {
    pub fn new(tokens: Vec<Ref<'a, Token>>, node: &'a LinkedNode) -> Self {
        Self { node, tokens }
    }
}

impl fmt::Display for NodeStep<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} [{} tokens]:{}",
            self.tokens
                .first()
                .map(|tk| tk.pos.from)
                .unwrap_or_default(),
            self.tokens.last().map(|tk| tk.pos.from).unwrap_or_default(),
            self.tokens.len(),
            self.node.ident()
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

    pub fn prev_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let mut tokens = Vec::new();
        loop {
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
                return None;
            }
            let Some(token) = self.parser.get_token_by_pos(self.pos) else {
                return None;
            };
            self.fingerprint = Some(token.fingerprint());
            tokens.push(token);
            if let Some(node) = find_node(self.anchor.childs(), &self.src, self.pos) {
                return Some(NodeStep::new(tokens, node));
            }
        }
    }

    pub fn next_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let mut tokens = Vec::new();
        let len = self.parser.len();
        loop {
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
                return None;
            }
            let Some(token) = self.parser.get_token_by_pos(self.pos) else {
                return None;
            };
            self.fingerprint = Some(token.fingerprint());
            tokens.push(token);
            if let Some(node) = find_node(self.anchor.childs(), &self.src, self.pos) {
                return Some(NodeStep::new(tokens, node));
            }
        }
    }

    pub fn prev_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
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

    pub fn next_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
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

    pub fn find<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let node = find_node(self.anchor.childs(), &self.src, self.pos);
        let Some(token) = self.parser.get_token_by_pos(self.pos) else {
            return None;
        };
        self.fingerprint = Some(token.fingerprint());
        Some(TokenStep::new(token, node))
    }
}
