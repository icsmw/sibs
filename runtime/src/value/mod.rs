/// Runtime Value
#[enum_ids::enum_ids(display_variant)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RtValue {
    Void,
    ExecuteResult,
    Range,
    Num,
    Bool,
    PathBuf,
    Str,
    Vec(Box<RtValue>),
    Error,
    Closure,
}
