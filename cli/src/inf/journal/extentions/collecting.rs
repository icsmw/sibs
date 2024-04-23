use crate::inf::{journal::api::Demand, Journal, Level};

#[derive(Debug)]
pub struct Collecting<'a> {
    bound: &'a Journal,
}
impl<'a> Collecting<'a> {
    pub fn new(bound: &'a Journal) -> Self {
        Self { bound }
    }

    pub fn append(&self, id: usize, msg: String) {
        if let Err(_err) = self.bound.tx.send(Demand::Collect(id, msg)) {
            eprintln!("Fail to store report; printing instead");
        }
    }

    pub fn close(&self, owner: String, id: usize, level: Level) {
        if let Err(_err) = self
            .bound
            .tx
            .send(Demand::CollectionClose(owner, id, level))
        {
            eprintln!("Fail to store report; printing instead");
        }
    }
}
