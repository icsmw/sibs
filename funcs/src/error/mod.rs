use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Function \"{0}\" has been registred already")]
    FuncAlreadyRegistered(String),
}
