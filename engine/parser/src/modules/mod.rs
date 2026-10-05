mod store;

use std::sync::Arc;

use asttree::ModuleBody;
pub(crate) use store::*;

use crate::*;

pub enum ModuleLoad {
    Cached(Arc<ModuleBody>),
    Unparsed { parser: Parser, path: PathBuf },
}
