use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum E {
    #[error("Token {0} not found")]
    TokenNotFound(usize),
    #[error("Token {0} has invalid range; string len={1}; range [{2},{3}]")]
    TokenHasInvalidRange(usize, usize, usize, usize),
}

// impl From<scenario::E> for E {
//     fn from(e: scenario::E) -> Self {
//         E::ScenarionError(e)
//     }
// }

// impl From<executors::E> for E {
//     fn from(e: executors::E) -> Self {
//         E::ExecutorsError(e)
//     }
// }

// impl From<reader::E> for E {
//     fn from(e: reader::E) -> Self {
//         E::ReaderError(e.to_string())
//     }
// }
