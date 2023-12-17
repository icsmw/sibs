use crate::reader::{
    entry::{Cmp, Reader, Reading, VariableName},
    words, E,
};

#[derive(Debug)]
pub struct VariableComparing {
    pub name: VariableName,
    pub cmp: Cmp,
    pub value: String,
    pub index: usize,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, E> {
        reader.hold();
        let from = reader.pos;
        if let Some(name) = VariableName::read(reader)? {
            if let Some(word) = reader.move_to_word(&[words::CMP_TRUE, words::CMP_FALSE])? {
                if reader.rest().trim().is_empty() {
                    Err(E::NoValueAfterComparing)
                } else {
                    Ok(Some(VariableComparing {
                        name,
                        cmp: if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value: reader.rest().trim().to_string(),
                        index: reader.get_index_until_current(from),
                    }))
                }
            } else {
                reader.roll_back();
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
