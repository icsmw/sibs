use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Not supported type: {0}")]
    NotSupportedType(String),
}
