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
            self.token.pos.from.abs,
            self.token.pos.to.abs,
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
                .map(|tk| tk.pos.from.abs)
                .unwrap_or_default(),
            self.tokens
                .last()
                .map(|tk| tk.pos.from.abs)
                .unwrap_or_default(),
            self.tokens.len(),
            self.node.ident()
        )
    }
}

pub struct LocationIterator<'a> {
    anchor: &'a Anchor,
    src: Uuid,
    idx: isize,
    initial: isize,
    recent: Option<Uuid>,
    pub parser: &'a Parser,
}

impl<'a> LocationIterator<'a> {
    pub fn new(anchor: &'a Anchor, src: Uuid, idx: usize, parser: &'a Parser) -> Self {
        Self {
            anchor,
            src,
            idx: idx as isize,
            initial: idx as isize,
            recent: None,
            parser,
        }
    }

    pub fn pin(&self) -> impl Fn(&mut LocationIterator) {
        let idx = self.idx;
        move |loc: &mut LocationIterator| {
            loc.idx = idx;
        }
    }

    pub fn drop(&mut self) {
        self.idx = self.initial;
    }

    pub fn find(&self, uuid: &Uuid) -> Option<&'a LinkedNode> {
        fn find<'a>(uuid: &Uuid, nodes: Vec<&'a LinkedNode>) -> Option<&'a LinkedNode> {
            if let Some(node) = nodes.iter().find(|n| n.uuid() == uuid) {
                Some(&node)
            } else {
                for node in nodes.into_iter() {
                    if let Some(node) = find(uuid, node.childs()) {
                        return Some(node);
                    }
                }
                None
            }
        }
        if &self.anchor.uuid == uuid {
            None
        } else {
            find(uuid, self.anchor.childs())
        }
    }

    pub fn get_ownership_tree(&self, pos: usize) -> Vec<&LinkedNode> {
        get_ownership_tree(self.anchor.childs(), &self.src, pos)
    }

    pub fn prev_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.parser.get_token(self.idx)?;
            if let Some(node) = find_node(self.anchor.childs(), &self.src, &token) {
                if self
                    .recent
                    .as_ref()
                    .map(|recent| recent != node.uuid())
                    .unwrap_or(true)
                {
                    tokens.push(token);
                    self.recent = Some(*node.uuid());
                    return Some(NodeStep::new(tokens, node));
                }
            }
            tokens.push(token);
            self.idx -= 1;
        }
    }

    pub fn next_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.parser.get_token(self.idx)?;
            if let Some(node) = find_node(self.anchor.childs(), &self.src, &token) {
                if self
                    .recent
                    .as_ref()
                    .map(|recent| recent != node.uuid())
                    .unwrap_or(true)
                {
                    tokens.push(token);
                    self.recent = Some(*node.uuid());
                    return Some(NodeStep::new(tokens, node));
                }
            }
            tokens.push(token);
            self.idx += 1;
        }
    }

    pub fn prev_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let token = self.parser.get_token(self.idx)?;
        let node = find_node(self.anchor.childs(), &self.src, &token);
        self.idx -= 1;
        Some(TokenStep::new(token, node))
    }

    pub fn next_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let token = self.parser.get_token(self.idx)?;
        let node = find_node(self.anchor.childs(), &self.src, &token);
        self.idx += 1;
        Some(TokenStep::new(token, node))
    }
    pub fn prev<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let token = loop {
            let token = self.parser.get_token(self.idx)?;
            if matches!(
                token.id(),
                KindId::BOF
                    | KindId::Whitespace
                    | KindId::LF
                    | KindId::CR
                    | KindId::CRLF
                    | KindId::EOF
            ) {
                self.idx -= 1;
                continue;
            } else {
                break token;
            }
        };
        let node = find_node(self.anchor.childs(), &self.src, &token);
        self.idx -= 1;
        Some(TokenStep::new(token, node))
    }

    pub fn next<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let token = loop {
            let token = self.parser.get_token(self.idx)?;
            if matches!(
                token.id(),
                KindId::BOF
                    | KindId::Whitespace
                    | KindId::LF
                    | KindId::CR
                    | KindId::CRLF
                    | KindId::EOF
            ) {
                self.idx += 1;
                continue;
            } else {
                break token;
            }
        };
        let node = find_node(self.anchor.childs(), &self.src, &token);
        self.idx += 1;
        Some(TokenStep::new(token, node))
    }
}
