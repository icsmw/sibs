use lexer::{LinkedPosition, Token};

pub enum SemanticToken {
    Keyword,
    Function,
    Variable,
    String,
    Number,
    Bool,
    Type,
    Parameter,
    Operator,
    Comment,
    Meta,
    Event,
    Class,
    /// Name of module for example
    Namespace,
    /// ', ", `
    Delimiter,
    Task,
    Component,
    Module,
}

#[derive(Debug, Clone, Copy)]
pub enum SemanticTokenContext {
    ArgumentDeclaration,
    VariableDeclaration,
    FunctionCall,
    Ignored,
}

pub struct LinkedSemanticToken {
    pub token: SemanticToken,
    pub position: LinkedPosition,
}

impl LinkedSemanticToken {
    pub fn from_token(source: &Token, token: SemanticToken) -> Self {
        LinkedSemanticToken {
            token,
            position: source.into(),
        }
    }
    pub fn between_tokens(left: &Token, right: &Token, token: SemanticToken) -> Self {
        LinkedSemanticToken {
            token,
            position: LinkedPosition {
                from: left.pos.from,
                to: right.pos.to,
                src: left.src,
            },
        }
    }
}

pub trait SemanticTokensGetter {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken>;
}
