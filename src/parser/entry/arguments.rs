use crate::parser::{
    chars,
    entry::{Group, Reader, Reading, ValueString},
    E,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Argument {
    String(String),
    ValueString(ValueString),
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub inner: Vec<Vec<(Uuid, Argument)>>,
}

impl Reading<Arguments> for Arguments {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut args = Arguments::new();
        while let Some(group) = Group::read(reader)? {
            args.add_from_group(group, reader)?;
        }
        if !reader.rest().is_empty() {
            args.add_args(reader.to_end().1, reader)?;
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
    pub fn read_string_args(reader: &mut Reader) -> Result<Vec<(Uuid, Argument)>, E> {
        let mut arguments: Vec<(Uuid, Argument)> = vec![];
        while let Some((arg, _, uuid)) = reader.read_until_wt(true)? {
            if !arg.trim().is_empty() {
                if reader.inherit(arg.clone()).has_char(chars::AT)? {
                    Err(E::NestedFunction)?
                }
                arguments.push((uuid, Argument::String(Reader::unserialize(&arg))));
            }
        }
        if !reader.rest().trim().is_empty() {
            if reader.has_char(chars::AT)? {
                Err(E::NestedFunction)?
            }
            let (uuid, rest) = reader.to_end();
            arguments.push((uuid, Argument::String(Reader::unserialize(&rest))));
        }
        Ok(arguments)
    }
    pub fn add_from_group(&mut self, group: Group, parent: &mut Reader) -> Result<(), E> {
        self.add_args(group.inner, parent)?;
        Ok(())
    }
    pub fn add_args(&mut self, inner: String, parent: &mut Reader) -> Result<(), E> {
        let mut reader = parent.inherit(inner);
        let mut arguments: Vec<(Uuid, Argument)> = vec![];
        loop {
            if let Some((before, _, _)) = reader.read_until(&[chars::QUOTES], false, false)? {
                arguments = [
                    arguments,
                    Arguments::read_string_args(&mut reader.inherit(before))?,
                ]
                .concat();
                if let Some(value_string) = ValueString::read(&mut reader)? {
                    arguments.push((Uuid::new_v4(), Argument::ValueString(value_string)));
                } else {
                    Err(E::NoStringEnd)?
                }
            } else {
                arguments = [arguments, Arguments::read_string_args(&mut reader)?].concat();
                break;
            }
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
