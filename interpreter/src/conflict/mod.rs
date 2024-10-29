mod cfm;
mod declaration;
mod expression;
mod miscellaneous;
mod root;
mod statement;
mod value;

pub use cfm::*;
pub use declaration::*;
pub use expression::*;
pub use miscellaneous::*;
pub use root::*;
pub use statement::*;
pub use value::*;

use crate::*;

pub trait ConflictResolver<K> {
    fn resolve_conflict(&self, id: &K) -> K;
}
