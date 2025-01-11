mod anchor;
mod component;
mod module;
mod task;

pub use anchor::*;
pub use component::*;
pub use module::*;
pub use task::*;

use crate::*;

#[enum_ids::enum_ids(derive = "Debug, PartialEq, Clone", display, display_from_value)]
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Root {
    /// The root document to start parsing
    Anchor(Anchor),
    /// Functions module
    Module(Module),
    /// component name() { ... }, component name(pwd) { ... }
    Component(Component),
    /// task name() { ... }, private task name(arg: string, ...) { ... }
    Task(Task),
}

impl Root {
    pub fn uuid(&self) -> &Uuid {
        match self {
            Self::Anchor(n) => &n.uuid,
            Self::Module(n) => &n.uuid,
            Self::Component(n) => &n.uuid,
            Self::Task(n) => &n.uuid,
        }
    }
}

impl SrcLinking for Root {
    fn link(&self) -> SrcLink {
        match self {
            Self::Anchor(n) => n.link(),
            Self::Module(n) => n.link(),
            Self::Component(n) => n.link(),
            Self::Task(n) => n.link(),
        }
    }
    fn slink(&self) -> SrcLink {
        self.link()
    }
}

impl From<Root> for Node {
    fn from(val: Root) -> Self {
        Node::Root(val)
    }
}
