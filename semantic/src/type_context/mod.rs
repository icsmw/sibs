use crate::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct TypeContext {
    pub symbols: HashMap<Uuid, DataType>,
}
