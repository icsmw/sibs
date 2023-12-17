use crate::reader::{
    chars,
    entry::{Function, Group, Reader, Reading, ValueString},
    E,
};

#[derive(Debug, Clone)]
pub enum Argument {
    String(String),
    ValueString(ValueString),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub args: Vec<(usize, Argument)>,
    pub index: usize,
}

impl Reading<Arguments> for Arguments {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut args = Arguments {
            args: vec![],
            index: 0,
        };
        let from = reader.pos;
        while let Some(group) = Group::read(reader)? {
            args.add_from_group(group, reader)?;
        }
        if !reader.rest().is_empty() {
            args.add_args(reader.to_end().1, reader)?;
        }
        if args.is_empty() {
            Ok(None)
        } else {
            args.index = reader.get_index_until_current(from);
            Ok(Some(args))
        }
    }
}

impl Arguments {
    pub fn read_string_args(reader: &mut Reader) -> Result<Vec<(usize, Argument)>, E> {
        let mut arguments: Vec<(usize, Argument)> = vec![];
        while let Some((arg, _, index)) = reader.read_until_wt(true)? {
            if !arg.trim().is_empty() {
                if reader.inherit(arg.clone()).has_char(chars::AT)? {
                    Err(E::NestedFunction)?
                }
                arguments.push((index, Argument::String(Reader::unserialize(&arg))));
            }
        }
        if !reader.rest().trim().is_empty() {
            if reader.has_char(chars::AT)? {
                Err(E::NestedFunction)?
            }
            let (index, rest) = reader.to_end();
            arguments.push((index, Argument::String(Reader::unserialize(&rest))));
        }
        Ok(arguments)
    }
    pub fn add_from_group(&mut self, group: Group, parent: &mut Reader) -> Result<(), E> {
        self.add_args(group.inner, parent)?;
        Ok(())
    }
    pub fn add_args(&mut self, inner: String, parent: &mut Reader) -> Result<(), E> {
        let mut reader = parent.inherit(inner);
        let mut arguments: Vec<(usize, Argument)> = vec![];
        loop {
            if let Some((before, _, _)) = reader.read_until(&[chars::QUOTES], false, false)? {
                arguments = [
                    arguments,
                    Arguments::read_string_args(&mut reader.inherit(before))?,
                ]
                .concat();
                if let Some(value_string) = ValueString::read(&mut reader)? {
                    arguments.push((0, Argument::ValueString(value_string)));
                } else {
                    Err(E::NoStringEnd)?
                }
            } else {
                arguments = [arguments, Arguments::read_string_args(&mut reader)?].concat();
                break;
            }
        }
        if !arguments.is_empty() {
            self.args = [self.args.clone(), arguments].concat();
        }
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
    pub fn add_fn_arg(&mut self, fn_arg: Function) {
        if let Some((_, arg)) = self
            .args
            .iter_mut()
            .find(|(_, arg)| matches!(arg, Argument::Function(_)))
        {
            if let Argument::Function(func) = arg {
                func.add_fn_arg(fn_arg);
            }
        } else {
            self.args.insert(0, (0, Argument::Function(fn_arg)));
        }
    }
}
