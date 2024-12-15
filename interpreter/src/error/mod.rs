use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Token isn't bound to known DataType")]
    TokenIsNotBoundToKnownDataType,
}
