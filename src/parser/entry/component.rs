use std::{path::PathBuf, str::FromStr};

use crate::parser::{chars, entry::Group, Reader, E};

#[derive(Debug)]
pub struct Component {
    pub cwd: Option<PathBuf>,
}

impl Component {
    pub fn new(group: Group, parent: &mut Reader) -> Result<Self, E> {
        let mut reader = parent.inherit(group.inner);
        let (name, stopped) = reader
            .read_letters_to_end(&[chars::COLON], false)?
            .ok_or(E::NoComponentContext)?;
        if name.is_empty() {
            Err(E::UnnamedComponent)?;
        }
        if let Some(stopped) = stopped {
            if stopped != chars::COLON {
                Err(E::UnexpectedChar(stopped))?;
            }
            // Absolute context
            Ok(Self {
                cwd: Some(PathBuf::from_str(reader.rest().trim())?),
            })
        } else {
            // Relative context
            Ok(Self { cwd: None })
        }
    }
}
