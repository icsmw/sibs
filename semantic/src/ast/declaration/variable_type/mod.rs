use crate::*;

use lexer::{Keyword, Kind, Token};

impl InferType for Token {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match &self.kind {
            Kind::Number(..) => Ok(DataType::Num),
            Kind::Keyword(Keyword::Bool) => Ok(DataType::Bool),
            Kind::Keyword(Keyword::Str) => Ok(DataType::Str),
            Kind::Keyword(Keyword::Num) => Ok(DataType::Num),
            _ => Err(LinkedErr::token(E::TokenIsNotBoundToKnownDataType, self)),
        }
    }
}

impl InferType for VariableCompoundType {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(_, n) => Ok(DataType::Vec(Box::new(n.infer_type(scx)?))),
        }
    }
}

impl InferType for VariableTypeDef {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(tk) => tk.infer_type(scx),
            VariableTypeDef::Compound(ty) => ty.infer_type(scx),
        }
    }
}

impl InferType for VariableType {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(scx)
    }
}

impl Initialize for VariableCompoundType {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(_, n) => n.initialize(scx),
        }
    }
}

impl Initialize for VariableTypeDef {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(..) => Ok(()),
            VariableTypeDef::Compound(ty) => ty.initialize(scx),
        }
    }
}

impl Initialize for VariableType {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(scx)
    }
}
