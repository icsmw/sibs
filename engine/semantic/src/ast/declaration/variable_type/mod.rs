use crate::*;

use lexer::{Keyword, Kind, Token};

impl InferType for Token {
    fn infer_type(&self, _scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match &self.kind {
            Kind::Number(..) => Ok(DeterminedTy::Num.into()),
            Kind::Keyword(Keyword::Bool) => Ok(DeterminedTy::Bool.into()),
            Kind::Keyword(Keyword::Str) => Ok(DeterminedTy::Str.into()),
            Kind::Keyword(Keyword::Num) => Ok(DeterminedTy::Num.into()),
            _ => Err(LinkedErr::token(E::TokenIsNotBoundToKnownTy, self)),
        }
    }
}

impl InferType for VariableCompoundType {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(tk, n) => {
                let inner = n.infer_type(scx)?;
                let inner = inner
                    .determined()
                    .cloned()
                    .ok_or(LinkedErr::token(E::FailInferDeterminedType(inner), tk))?;
                Ok(DeterminedTy::Vec(Some(Box::new(inner))).into())
            }
        }
    }
}

impl InferType for VariableTypeDef {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(tk) => tk.infer_type(scx),
            VariableTypeDef::Compound(ty) => ty.infer_type(scx),
        }
    }
}

impl InferType for VariableType {
    fn infer_type(&self, scx: &mut SemanticCx) -> Result<Ty, LinkedErr<E>> {
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

impl Finalization for VariableCompoundType {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            VariableCompoundType::Vec(_, n) => n.finalize(scx),
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

impl Finalization for VariableTypeDef {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        match self {
            VariableTypeDef::Primitive(..) => Ok(()),
            VariableTypeDef::Compound(ty) => ty.finalize(scx),
        }
    }
}

impl Initialize for VariableType {
    fn initialize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.initialize(scx)?;
        let ty = self.r#type.infer_type(scx)?;
        scx.table.set(&self.uuid, ty);
        Ok(())
    }
}

impl Finalization for VariableType {
    fn finalize(&self, scx: &mut SemanticCx) -> Result<(), LinkedErr<E>> {
        self.r#type.finalize(scx)
    }
}
