#[cfg(test)]
use crate::elements::{Element, ElementRef};

pub trait TokenGetter {
    fn token(&self) -> usize;
}

#[cfg(test)]
pub trait ElementRefGetter {
    fn get_alias(&self) -> ElementRef;
}

#[cfg(test)]
pub trait InnersGetter {
    fn get_inners(&self) -> Vec<&Element>;
}
