use std::path::PathBuf;

use crate::parser::{
    chars,
    entry::{Group, Reading},
    Reader, E,
};

#[derive(Debug)]
pub struct Component {
    pub cwd: PathBuf,
    pub name: String,
}

impl Reading<Component> for Component {
    fn read(reader: &mut Reader) -> Result<Option<Component>, E> {
        if reader.move_to_char(&[chars::POUND_SIGN])?.is_some() {
            if let Some(group) = Group::read(reader)? {
                let mut inner = reader.inherit(group.inner);
                if let Some((name, _, _)) = inner.read_until(&[chars::COLON], true, false)? {
                    Ok(Some(Component {
                        name,
                        cwd: PathBuf::from(inner.rest().trim()),
                    }))
                } else {
                    Err(E::NoColon)
                }
            } else {
                Err(E::NoGroup)?
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{
        entry::{Component, Reading},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/component.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(comp) = Component::read(&mut reader)? {
            println!("{comp:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
