#[cfg(test)]
use crate::elements::Element;
use crate::elements::ElementId;

pub trait TokenGetter {
    fn token(&self) -> usize;
}

pub trait ElementRefGetter {
    fn id(&self) -> ElementId;
}

#[cfg(test)]
pub trait InnersGetter {
    fn get_inners(&self) -> Vec<&Element>;
}
