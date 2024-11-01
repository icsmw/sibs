use lexer::Token;

mod read;

#[derive(Debug, Clone)]
enum Side {
    Number(isize, Token),
    Variable(String, Token),
}

#[derive(Debug, Clone)]
pub struct Range {
    pub left: Side,
    pub right: Side,
}
