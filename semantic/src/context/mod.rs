use crate::*;

#[derive(Debug, Default)]
pub struct SemanticCx {
    pub tys: TyStore,
    pub fns: Fns,
    pub table: TypesTable,
}

impl SemanticCx {
    pub fn by_node<N: InferType + Identification>(&mut self, node: &N) -> Result<(), LinkedErr<E>> {
        if self.table.has(node.uuid()) {
            // It's PPM and it's already registred
            return Ok(());
        }
        let ty = node.infer_type(self)?;
        self.table.set(node.uuid(), ty);
        Ok(())
    }
    pub fn register(&mut self, uuid: &Uuid, ty: &DataType) {
        self.table.set(uuid, ty.clone());
    }
}
