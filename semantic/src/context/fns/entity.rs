use common::DataType;
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct FnEntity {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub args: Vec<DataType>,
    pub out: DataType,
}
