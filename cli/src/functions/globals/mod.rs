mod get_cwd;
mod get_var;
mod set_var;

use crate::{
    functions::{ExecutorFn, E},
    inf::Store,
};

pub fn register(store: &mut Store<ExecutorFn>) -> Result<(), E> {
    store.insert(get_cwd::name(), get_cwd::execute)?;
    store.insert(set_var::name(), set_var::execute)?;
    store.insert(get_var::name(), get_var::execute)?;
    Ok(())
}
