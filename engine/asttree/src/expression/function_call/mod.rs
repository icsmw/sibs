#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub args: Vec<LinkedNode>,
    pub reference: Vec<(String, Token)>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
    pub negation: Option<Token>,
}

impl FunctionCall {
    pub fn get_name(&self) -> String {
        self.reference
            .iter()
            .map(|(n, _)| n.to_owned())
            .collect::<Vec<String>>()
            .join("::")
    }
    pub fn get_last_name(&self) -> String {
        self.reference
            .last()
            .map(|(n, _)| n.clone())
            .unwrap_or_default()
    }
}

impl<'a> Lookup<'a> for FunctionCall {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .collect::<Vec<&LinkedNode>>()
            .lookup_inner(self.uuid, trgs)
    }
}

impl FindMutByUuid for FunctionCall {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.args.find_mut_by_uuid(uuid)
    }
}

impl SrcLinking for FunctionCall {
    fn link(&self) -> SrcLink {
        if let Some(open) = self.negation.as_ref() {
            src_from::tks(open, &self.close)
        } else if let Some((_, open)) = self.reference.first() {
            src_from::tks(open, &self.close)
        } else {
            src_from::tks(&self.open, &self.close)
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} {} {} {}",
            self.negation
                .as_ref()
                .map(|tk| format!("{tk} "))
                .unwrap_or_default(),
            self.reference
                .iter()
                .map(|(s, _)| s.to_owned())
                .collect::<Vec<String>>()
                .join(&Kind::Colon.to_string().repeat(2)),
            Kind::LeftParen,
            self.args
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightParen
        )
    }
}

impl From<FunctionCall> for Node {
    fn from(val: FunctionCall) -> Self {
        Node::Expression(Expression::FunctionCall(val))
    }
}
