mod get_cwd;
mod get_var;
mod set_cwd;
mod set_var;

use crate::{functions::Store, functions::E};

pub fn register(store: &mut Store) -> Result<(), E> {
    store.insert(set_cwd::name(), set_cwd::execute)?;
    store.insert(get_cwd::name(), get_cwd::execute)?;
    store.insert(set_var::name(), set_var::execute)?;
    store.insert(get_var::name(), get_var::execute)?;
    Ok(())
}
