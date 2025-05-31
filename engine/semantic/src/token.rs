use lexer::{LinkedPosition, Token};

#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn extract_by_relative<'a>(&self, content: &'a str) -> Option<&'a str> {
        let lines: Vec<&str> = content.split('\n').collect();
        let mut from_offset = 0;

        for ln in 0..self.position.from.ln {
            from_offset += lines.get(ln)?.len() + 1; // +1 for \n
        }
        from_offset += self.position.from.col;

        let mut to_offset = 0;
        for ln in 0..self.position.to.ln {
            to_offset += lines.get(ln)?.len() + 1; // +1 for \n
        }
        to_offset += self.position.to.col;
        println!("absolute cors based on ln,col: (from: {from_offset}; to: {to_offset})",);
        content.get(from_offset..to_offset)
    }
}

pub trait SemanticTokensGetter {
    fn get_semantic_tokens(&self, stcx: SemanticTokenContext) -> Vec<LinkedSemanticToken>;
}
