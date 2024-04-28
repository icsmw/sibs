use uuid::Uuid;

use crate::inf::{journal::api::Demand, Journal, Level};

#[derive(Debug)]
pub struct Collecting<'a> {
    bound: &'a Journal,
}
impl<'a> Collecting<'a> {
    pub fn new(bound: &'a Journal) -> Self {
        Self { bound }
    }

    pub fn append(&self, uuid: Uuid, msg: String) {
        if let Err(_err) = self.bound.tx.send(Demand::Collect(uuid, msg)) {
            eprintln!("Fail to store report; printing instead");
        }
    }

    pub fn close(&self, owner: String, uuid: Uuid, level: Level) {
        if let Err(_err) = self
            .bound
            .tx
            .send(Demand::CollectionClose(owner, uuid, level))
        {
            eprintln!("Fail to store report; printing instead");
        }
    }
}
