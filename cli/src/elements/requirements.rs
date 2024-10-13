#[cfg(test)]
use crate::elements::{Element, ElementId};

pub trait TokenGetter {
    fn token(&self) -> usize;
}

#[cfg(test)]
pub trait ElementRefGetter {
    fn get_alias(&self) -> ElementId;
}

#[cfg(test)]
pub trait InnersGetter {
    fn get_inners(&self) -> Vec<&Element>;
}
