use crate::*;

pub trait Diagnostic {
    fn located(&self, src: &Uuid, pos: usize) -> bool;
    fn get_position(&self) -> Position;
    fn childs(&self) -> Vec<&LinkedNode>;
}
