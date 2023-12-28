use crate::reader::{
    entry::{Cmp, Reader, Reading, VariableName},
    words, E,
};

#[derive(Debug)]
pub struct VariableComparing {
    pub name: VariableName,
    pub cmp: Cmp,
    pub value: String,
    pub token: usize,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, E> {
        reader.state().set();
        if let Some(name) = VariableName::read(reader)? {
            if let Some(word) = reader
                .move_to()
                .word(&[&words::CMP_TRUE, &words::CMP_FALSE])
            {
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
                        value: reader.move_to().end().trim().to_string(),
                        token: reader.token()?.id,
                    }))
                }
            } else {
                reader.state().restore()?;
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
