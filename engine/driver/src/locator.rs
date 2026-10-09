use std::ops::RangeInclusive;

use interpreter::InterContext;

use crate::*;

pub struct TokenStep<'a> {
    pub node: Option<&'a LinkedNode>,
    pub token: &'a Token,
    pub idx: isize,
}

impl<'a> TokenStep<'a> {
    pub fn new(token: &'a Token, node: Option<&'a LinkedNode>, idx: isize) -> Self {
        Self { node, token, idx }
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
            self.token.kind.id(),
            self.node
                .map(|n| format!("{}:{}", n.ident(), n.uuid()))
                .unwrap_or(String::from("None"))
        )
    }
}

pub struct NodeStep<'a> {
    pub node: &'a LinkedNode,
    pub tokens: Vec<&'a Token>,
}

impl<'a> NodeStep<'a> {
    pub fn new(tokens: Vec<&'a Token>, node: &'a LinkedNode) -> Self {
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
    src: Uuid,
    pub idx: isize,
    initial: isize,
    recent: Option<Uuid>,
    pub ctx: &'a InterContext,
}

impl<'a> LocationIterator<'a> {
    pub fn new(src: Uuid, idx: usize, ctx: &'a InterContext) -> Self {
        Self {
            src,
            idx: idx as isize,
            initial: idx as isize,
            recent: None,
            ctx,
        }
    }

    pub fn pin(&self) -> impl Fn(&mut LocationIterator) + use<> {
        let idx = self.idx;
        move |loc: &mut LocationIterator| {
            loc.idx = idx;
        }
    }

    pub fn drop(&mut self) {
        self.idx = self.initial;
    }

    pub fn set_idx(&mut self, idx: isize) {
        self.idx = idx;
    }

    pub fn nth_token(&self, idx: isize) -> Option<&Token> {
        self.tokens()?.get(idx)
    }

    pub fn nth_tokens(&self, range: RangeInclusive<usize>) -> Vec<Option<&Token>> {
        let mut tokens = Vec::new();
        let Some(stream) = self.tokens() else {
            return tokens;
        };
        for idx in range {
            tokens.push(stream.get(idx as isize));
        }
        tokens
    }

    pub fn find(&self, uuid: &Uuid) -> Option<&'a LinkedNode> {
        fn find<'a>(uuid: &Uuid, nodes: Vec<&'a LinkedNode>) -> Option<&'a LinkedNode> {
            if let Some(node) = nodes.iter().find(|n| n.uuid() == uuid) {
                Some(node)
            } else {
                for node in nodes.into_iter() {
                    if let Some(node) = find(uuid, node.childs()) {
                        return Some(node);
                    }
                }
                None
            }
        }
        let anchor = self.ctx.get_anchor()?;
        find(uuid, vec![anchor])
    }

    pub fn get_ownership_tree(&self, pos: usize) -> Vec<&LinkedNode> {
        let Some(anchor) = self.ctx.get_anchor() else {
            return Vec::new();
        };
        let owner = self
            .tokens()
            .and_then(|tokens| tokens.get_by_pos(pos))
            .and_then(|(token, _)| token.owner.as_ref().map(|(owner, _)| owner));
        get_ownership_tree(vec![anchor], &self.src, pos, owner)
    }

    pub fn prev_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let anchor = self.ctx.get_anchor()?;
        let stream = self.tokens()?;
        let mut tokens = Vec::new();
        loop {
            let token = stream.get(self.idx)?;
            if let Some(node) = find_node(vec![anchor], token)
                && self
                    .recent
                    .as_ref()
                    .map(|recent| recent != node.uuid())
                    .unwrap_or(true)
            {
                tokens.push(token);
                self.recent = Some(*node.uuid());
                return Some(NodeStep::new(tokens, node));
            }
            tokens.push(token);
            self.idx -= 1;
        }
    }

    pub fn next_node<'s>(&'s mut self) -> Option<NodeStep<'s>> {
        let anchor = self.ctx.get_anchor()?;
        let stream = self.tokens()?;
        let mut tokens = Vec::new();
        loop {
            let token = stream.get(self.idx)?;
            if let Some(node) = find_node(vec![anchor], token)
                && self
                    .recent
                    .as_ref()
                    .map(|recent| recent != node.uuid())
                    .unwrap_or(true)
            {
                tokens.push(token);
                self.recent = Some(*node.uuid());
                return Some(NodeStep::new(tokens, node));
            }
            tokens.push(token);
            self.idx += 1;
        }
    }

    pub fn prev_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let anchor = self.ctx.get_anchor();
        let stream = self.tokens()?;
        let token = stream.get(self.idx)?;
        let node = anchor.and_then(|anchor| find_node(vec![anchor], token));
        self.idx -= 1;
        Some(TokenStep::new(token, node, self.idx + 1))
    }

    pub fn next_token<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let anchor = self.ctx.get_anchor();
        let stream = self.tokens()?;
        let token = stream.get(self.idx)?;
        let node = anchor.and_then(|anchor| find_node(vec![anchor], token));
        self.idx += 1;
        Some(TokenStep::new(token, node, self.idx - 1))
    }
    pub fn prev<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let anchor = self.ctx.get_anchor();
        let stream = self.tokens()?;
        let token = loop {
            let token = stream.get(self.idx)?;
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
        let node = anchor.and_then(|anchor| find_node(vec![anchor], token));
        self.idx -= 1;
        Some(TokenStep::new(token, node, self.idx + 1))
    }

    pub fn next<'s>(&'s mut self) -> Option<TokenStep<'s>> {
        let anchor = self.ctx.get_anchor();
        let stream = self.tokens()?;
        let token = loop {
            let token = stream.get(self.idx)?;
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
        let node = anchor.and_then(|anchor| find_node(vec![anchor], token));
        self.idx += 1;
        Some(TokenStep::new(token, node, self.idx - 1))
    }

    pub fn prev_find_id<P>(&mut self, mut predicate: P) -> Option<KindId>
    where
        P: FnMut(&TokenStep) -> bool,
    {
        while let Some(prev) = self.prev() {
            if predicate(&prev) {
                return Some(prev.token.id());
            }
        }
        None
    }

    fn tokens(&self) -> Option<&'a Tokens> {
        self.ctx.get_diagnostics()?.tokens().get(&self.src)
    }
}
