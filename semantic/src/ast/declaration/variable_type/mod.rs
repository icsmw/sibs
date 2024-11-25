use crate::*;

use lexer::{Keyword, Kind, Token};

impl InferType for Token {
    fn infer_type(&self, _tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match &self.kind {
            Kind::Number(..) => Ok(DataType::F64),
            Kind::Keyword(Keyword::Bool) => Ok(DataType::Bool),
            Kind::Keyword(Keyword::Str) => Ok(DataType::Str),
            Kind::Keyword(Keyword::Num) => Ok(DataType::F64),
            _ => Err(LinkedErr::token(E::TokenIsNotBoundToKnownDataType, self)),
        }
    }
}

impl InferType for VariableCompoundType {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(_, n) => Ok(DataType::Vec(Box::new(n.infer_type(tcx)?))),
        }
    }
}

impl InferType for VariableTypeDef {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(tk) => tk.infer_type(tcx),
            VariableTypeDef::Compound(ty) => ty.infer_type(tcx),
        }
    }
}

impl InferType for VariableType {
    fn infer_type(&self, tcx: &mut TypeContext) -> Result<DataType, LinkedErr<E>> {
        self.r#type.infer_type(tcx)
    }
}

impl Initialize for VariableCompoundType {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(_, n) => n.initialize(tcx),
        }
    }
}

impl Initialize for VariableTypeDef {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(..) => Ok(()),
            VariableTypeDef::Compound(ty) => ty.initialize(tcx),
        }
    }
}

impl Initialize for VariableType {
    fn initialize(&self, tcx: &mut TypeContext) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(tcx)
    }
}
