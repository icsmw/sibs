use crate::inf::AnyValue;

pub const TRACE_DEFAILT_DEEP: usize = 2;

type Footprint = (usize, String);

#[derive(Debug)]
pub struct Trace {
    footprints: Vec<Footprint>,
    deep: usize,
}

impl Trace {
    pub fn new(deep: Option<usize>) -> Self {
        Self {
            footprints: Vec::new(),
            deep: deep.unwrap_or(TRACE_DEFAILT_DEEP),
        }
    }
    pub fn add(&mut self, token: &usize, value: &Option<AnyValue>) {
        println!(">>>>>>>>>>>>>>>>>>>> ADD: {value:?}");
        if self.deep != 0 {
            self.footprints
                .push((*token, format!("{:?}", format!("{value:?}"))));
            if self.footprints.len() > self.deep && !self.footprints.is_empty() {
                self.footprints.remove(0);
            }
        }
    }
    pub fn iter(&self) -> core::slice::Iter<'_, Footprint> {
        self.footprints.iter()
    }
}
