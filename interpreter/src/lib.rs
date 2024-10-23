mod lexer;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use tests::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
