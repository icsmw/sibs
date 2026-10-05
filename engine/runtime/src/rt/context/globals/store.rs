use crate::*;
use std::panic::{catch_unwind, AssertUnwindSafe};

struct GlobalValue {
    value: Arc<RtValue>,
    ty: Ty,
    mutable: bool,
    link: SrcLink,
}

/// Owned exclusively by the execution context's request handler.
#[derive(Default)]
pub(in crate::rt::context) struct GlobalValues {
    values: HashMap<String, GlobalValue>,
}

impl GlobalValues {
    pub fn lookup(&self, name: &str) -> Option<Arc<RtValue>> {
        self.values.get(name).map(|entry| entry.value.clone())
    }

    pub fn is_registered(&self, name: &str, link: &SrcLink) -> bool {
        self.values
            .get(name)
            .is_some_and(|entry| &entry.link == link)
    }

    pub fn register(
        &mut self,
        name: String,
        ty: Ty,
        mutable: bool,
        link: SrcLink,
        value: RtValue,
    ) -> Result<(), E> {
        if !value.is_compatible(&ty) {
            return Err(E::InvalidValueType(ty.to_string()));
        }
        if let Some(previous) = self.values.get(&name) {
            if previous.link == link && previous.ty == ty && previous.mutable == mutable {
                return Ok(());
            }
            return Err(E::GlobalConflict(name));
        }
        self.values.insert(
            name,
            GlobalValue {
                value: Arc::new(value),
                ty,
                mutable,
                link,
            },
        );
        Ok(())
    }

    pub fn update(
        &mut self,
        name: &str,
        transform: impl FnOnce(&RtValue) -> Result<RtValue, E>,
    ) -> Result<Arc<RtValue>, E> {
        let entry = self
            .values
            .get_mut(name)
            .ok_or_else(|| E::UndefinedVariable(name.into()))?;
        if !entry.mutable {
            return Err(E::ImmutableGlobal(name.into()));
        }
        // Only the returned value is committed; errors and panics preserve the current value.
        let value =
            catch_unwind(AssertUnwindSafe(|| transform(&entry.value))).map_err(|payload| {
                let message = if let Some(message) = payload.downcast_ref::<String>() {
                    message.clone()
                } else if let Some(message) = payload.downcast_ref::<&str>() {
                    (*message).to_owned()
                } else {
                    "non-string panic payload".to_owned()
                };
                E::ExecutionPanicked(message)
            })??;
        if !value.is_compatible(&entry.ty) {
            return Err(E::InvalidValueType(entry.ty.to_string()));
        }
        entry.value = Arc::new(value);
        Ok(entry.value.clone())
    }
}
