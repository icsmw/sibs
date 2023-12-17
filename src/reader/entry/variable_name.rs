use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub index: usize,
}

impl Reading<VariableName> for VariableName {
    fn read(reader: &mut Reader) -> Result<Option<VariableName>, E> {
        if reader.move_to_char(&[chars::DOLLAR])?.is_some() {
            if let Some((word, _, index)) =
                reader.read_letters(&[chars::COLON], &[chars::UNDERLINE], false)?
            {
                Ok(Some(VariableName::new(word, index)))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl VariableName {
    pub fn new(name: String, index: usize) -> Self {
        Self { name, index }
    }
}
