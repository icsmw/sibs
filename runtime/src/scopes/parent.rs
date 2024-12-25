use crate::*;

#[derive(Debug, Default)]
pub struct Parent {
    pub vl: Option<RtValue>,
}

impl Parent {
    pub fn set(&mut self, vl: RtValue) {
        self.vl = Some(vl);
    }
    pub fn withdraw(&mut self) -> Option<RtValue> {
        self.vl.take()
    }
    pub fn drop(&mut self) {
        self.vl = None;
    }
}
