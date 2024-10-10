mod executing;
mod formation;
mod interfaces;
#[cfg(test)]
mod proptests;
mod reading;
#[cfg(test)]
mod tests;
mod verification;

use crate::{
    elements::{Element, ElementRef},
    error::LinkedErr,
    inf::operator::E,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Block {
    pub elements: Vec<Element>,
    pub owner: Option<ElementRef>,
    pub breaker: Option<CancellationToken>,
    pub token: usize,
}

impl Block {
    pub fn set_owner(&mut self, owner: ElementRef) {
        self.owner = Some(owner);
    }
    pub fn set_breaker(&mut self, breaker: CancellationToken) {
        self.breaker = Some(breaker);
    }
    pub fn get_breaker(&self) -> Result<CancellationToken, LinkedErr<E>> {
        let Some(breaker) = self.breaker.as_ref() else {
            return Err(E::NoBreakSignalSetupForBlock.by(self));
        };
        Ok(breaker.clone())
    }
}
