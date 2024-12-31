use crate::*;

#[derive(Debug, Default)]
pub struct RtParent {
    pub vl: Option<RtValue>,
}

impl RtParent {
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
