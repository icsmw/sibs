use asttree::LinkedNode;
use lexer::{LinkedPosition, Token};
use std::fmt;

#[derive(Clone, Debug)]
pub struct LinkedErr<T: fmt::Display> {
    pub link: LinkedPosition,
    pub e: T,
}

impl<T: fmt::Display> LinkedErr<T> {
    pub fn token(err: T, token: &Token) -> Self {
        Self {
            link: token.into(),
            e: err,
        }
    }
    pub fn between(err: T, from: &Token, to: &Token) -> Self {
        Self {
            link: (from, to).into(),
            e: err,
        }
    }
    pub fn between_nodes(err: T, from: &LinkedNode, to: &LinkedNode) -> Self {
        Self {
            link: LinkedPosition {
                from: from.md.link.from(),
                to: to.md.link.to(),
                src: from.md.link.src,
            },
            e: err,
        }
    }
    pub fn by_pos(err: T, link: &LinkedPosition) -> Self {
        Self {
            link: link.to_owned(),
            e: err,
        }
    }
    pub fn by_link(err: T, link: LinkedPosition) -> Self {
        Self { link, e: err }
    }
    pub fn by_node(err: T, node: &LinkedNode) -> Self {
        Self {
            link: (&node.md.link).into(),
            e: err,
        }
    }
    pub fn unlinked(err: T) -> Self {
        Self {
            e: err,
            link: LinkedPosition::default(),
        }
    }

    pub fn is_unlinked(&self) -> bool {
        self.link.from == 0 && self.link.from == self.link.to
    }

    pub fn relink(&mut self, node: &LinkedNode) {
        self.link = (&node.md.link).into();
    }
}
