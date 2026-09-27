#[derive(Debug, Clone, Copy, Default)]
pub struct ScriptOptions {
    pub resilience: bool,
}

impl ScriptOptions {
    pub fn strict() -> Self {
        Self { resilience: false }
    }

    pub fn resilient() -> Self {
        Self { resilience: true }
    }
}
