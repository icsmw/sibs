use crate::parser::{
    chars,
    entry::{Reader, Reading},
    E,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct VariableName {
    pub name: String,
    pub uuid: Uuid,
}

impl Reading<VariableName> for VariableName {
    fn read(reader: &mut Reader) -> Result<Option<VariableName>, E> {
        if reader.move_to_char(chars::DOLLAR)? {
            if let Some((word, _, uuid)) =
                reader.read_letters(&[chars::COLON], &[chars::UNDERLINE], false, true)?
            {
                Ok(Some(VariableName::new(word, uuid)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableName {
    pub fn new(name: String, uuid: Uuid) -> Self {
        Self { name, uuid }
    }
}
