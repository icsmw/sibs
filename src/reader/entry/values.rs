use crate::reader::{
    chars,
    entry::{Reader, Reading},
    E,
};

#[derive(Debug)]
pub struct Values {
    pub values: Vec<String>,
    pub index: usize,
}

impl Reading<Values> for Values {
    fn read(reader: &mut Reader) -> Result<Option<Values>, E> {
        let from = reader.pos;
        if let Some((variants, _stopped_on, _uuid)) =
            reader.read_until(&[chars::SEMICOLON], true, true)?
        {
            Ok(Some(Values::new(
                variants,
                reader.get_index_until_current(from),
            )?))
        } else {
            Err(E::NoTypeDeclaration)
        }
    }
}

impl Values {
    pub fn new(input: String, index: usize) -> Result<Self, E> {
        let mut values: Vec<String> = vec![];
        for value in input.split('|') {
            let value = value.trim();
            if !value.is_ascii() {
                Err(E::NotAsciiValue(value.to_string()))?;
            }
            if value.is_empty() {
                Err(E::EmptyValue)?;
            }
            values.push(value.to_string());
        }
        if values.is_empty() {
            Err(E::NoVariableValues)?;
        }
        Ok(Values { values, index })
    }
}
