use crate::parser::{
    entry::{Group, Reader, Reading},
    E,
};

#[derive(Debug)]
pub struct Arguments {
    pub inner: Vec<Vec<String>>,
}

impl Reading<Arguments> for Arguments {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut args = Arguments::new();
        while let Some(group) = Group::read(reader)? {
            args.add_from_group(group)?;
        }
        if !reader.rest().is_empty() {
            args.add_args(reader.to_end())?;
        }
        if args.is_empty() {
            Ok(None)
        } else {
            Ok(Some(args))
        }
    }
}

impl Arguments {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }
    pub fn add_from_group(&mut self, group: Group) -> Result<(), E> {
        self.add_args(group.inner)?;
        Ok(())
    }
    pub fn add_args(&mut self, inner: String) -> Result<(), E> {
        let mut reader = Reader::new(inner);
        let mut arguments: Vec<String> = vec![];
        while let Some((arg, _)) = reader.read_until_wt(true)? {
            if !arg.trim().is_empty() {
                arguments.push(Reader::unserialize(&arg));
            }
        }
        if !reader.rest().trim().is_empty() {
            arguments.push(Reader::unserialize(&reader.to_end()));
        }
        if !arguments.is_empty() {
            self.inner.push(arguments);
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
