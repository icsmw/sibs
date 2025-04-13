#[cfg(feature = "proptests")]
mod proptests;

use crate::*;
use std::fmt;

#[derive(Debug, Clone)]
pub enum SkipTaskArgument {
    Value(LinkedNode),
    Any,
}

impl<'a> LookupInner<'a> for &'a SkipTaskArgument {
    fn lookup_inner(self, owner: Uuid, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        match self {
            SkipTaskArgument::Any => Vec::new(),
            SkipTaskArgument::Value(node) => node.lookup_inner(owner, trgs),
        }
    }
}

impl FindMutByUuid for SkipTaskArgument {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        if let Self::Value(node) = self {
            return if node.uuid() == uuid {
                Some(node)
            } else {
                node.node.find_mut_by_uuid(uuid)
            };
        }
        None
    }
}

impl fmt::Display for SkipTaskArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Value(n) => n.to_string(),
                Self::Any => Kind::Star.to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Skip {
    pub token: Token,
    pub args: Vec<SkipTaskArgument>,
    pub func: Box<LinkedNode>,
    pub open: Token,
    pub close: Token,
    pub uuid: Uuid,
}

impl<'a> Lookup<'a> for Skip {
    fn lookup(&'a self, trgs: &[NodeTarget]) -> Vec<FoundNode<'a>> {
        self.args
            .iter()
            .flat_map(|arg| arg.lookup_inner(self.uuid, trgs))
            .collect::<Vec<FoundNode>>()
            .into_iter()
            .chain(self.func.lookup_inner(self.uuid, trgs))
            .collect()
    }
}

impl FindMutByUuid for Vec<SkipTaskArgument> {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.iter_mut().find_map(|node| node.find_mut_by_uuid(uuid))
    }
}

impl FindMutByUuid for Skip {
    fn find_mut_by_uuid(&mut self, uuid: &Uuid) -> Option<&mut LinkedNode> {
        self.func
            .find_mut_by_uuid(uuid)
            .or_else(|| self.args.find_mut_by_uuid(uuid))
    }
}

impl SrcLinking for Skip {
    fn link(&self) -> SrcLink {
        src_from::tks(&self.token, &self.close)
    }
    fn slink(&self) -> SrcLink {
        src_from::tk(&self.token)
    }
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.token,
            self.open,
            Kind::LeftBracket,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(&format!(" {} ", Kind::Comma)),
            Kind::RightBracket,
            Kind::Comma,
            self.func,
            self.close
        )
    }
}

impl From<Skip> for Node {
    fn from(val: Skip) -> Self {
        Node::ControlFlowModifier(ControlFlowModifier::Skip(val))
    }
}
