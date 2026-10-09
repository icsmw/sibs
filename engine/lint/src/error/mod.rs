// Diagnostic names describe the problem, including in the generated EId enum.
#![allow(clippy::enum_variant_names)]

mod codes;

use enum_ids::enum_ids;
use thiserror::Error;

#[derive(Error, Debug)]
#[enum_ids(derive = "Debug")]
pub enum E {
    #[error("Component \"{0}\" has no documentation")]
    MissingComponentDocs(String),
    #[error("Task \"{0}\" has no documentation")]
    MissingTaskDocs(String),
    #[error("Task \"{0}\" argument \"{1}\" has no documentation")]
    MissingTaskArgumentDocs(String, String),
}
