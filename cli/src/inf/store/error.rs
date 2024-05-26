use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Item \"{0}\" already has been registred")]
    ItemAlreadyExists(String),
}
