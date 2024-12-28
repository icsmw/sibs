mod types;

use crate::*;
pub(crate) use types::*;

#[derive(Debug, Default)]
pub struct SemanticCx {
    pub tys: Types,
    pub fns: Fns,
    pub table: TypesTable,
}

impl SemanticCx {
    pub fn by_node<N: InferType + Identification>(&mut self, node: &N) -> Result<(), LinkedErr<E>> {
        let ty = node.infer_type(self)?;
        self.table.set(node.uuid(), ty);
        Ok(())
    }
    pub fn register(&mut self, uuid: &Uuid, ty: &DataType) {
        self.table.set(uuid, ty.clone());
        println!(">>>>>>>>>>>>>>>>>>:{:?}", self.table);
    }
}
