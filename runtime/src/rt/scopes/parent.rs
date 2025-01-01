use crate::*;

#[derive(Debug)]
pub struct ParentValue {
    pub value: RtValue,
    pub link: SrcLink,
}

impl ParentValue {
    pub fn by_node(value: RtValue, node: &LinkedNode) -> Self {
        Self {
            value,
            link: node.md.link.clone(),
        }
    }
}

impl From<ParentValue> for FnArgValue {
    fn from(value: ParentValue) -> Self {
        FnArgValue {
            value: value.value,
            link: value.link,
        }
    }
}

#[derive(Debug, Default)]
pub struct RtParent {
    pub vl: Option<ParentValue>,
}

impl RtParent {
    pub fn set(&mut self, vl: ParentValue) {
        self.vl = Some(vl);
    }
    pub fn withdraw(&mut self) -> Option<ParentValue> {
        self.vl.take()
    }
    pub fn drop(&mut self) {
        self.vl = None;
    }
}
