mod get_cwd;
mod set_cwd;

use crate::{executors::Store, executors::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(set_cwd::name(), set_cwd::execute)?;
    store.insert(get_cwd::name(), get_cwd::execute)?;
    Ok(())
}
