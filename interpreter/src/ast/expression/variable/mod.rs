use lexer::Token;

#[cfg(test)]
mod proptests;
mod read;

#[derive(Debug, Clone)]
pub struct Variable {
    pub ident: String,
    pub token: Token,
}
