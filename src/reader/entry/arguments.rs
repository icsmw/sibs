use crate::reader::{
    chars,
    entry::{Function, Reader, Reading, ValueString},
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
    pub token: usize,
}

impl Reading<Arguments> for Arguments {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut args = Arguments {
            args: vec![],
            token: 0,
        };
        while reader
            .group()
            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
            .is_some()
        {
            args.add_args(&mut reader.token()?.bound)?;
        }
        if !reader.move_to().end().is_empty() {
            args.add_args(&mut reader.token()?.bound)?;
        }
        if args.is_empty() {
            Ok(None)
        } else {
            args.token = reader.token()?.id;
            Ok(Some(args))
        }
    }
}

impl Arguments {
    pub fn read_string_args(reader: &mut Reader) -> Result<Vec<(usize, Argument)>, E> {
        let mut arguments: Vec<(usize, Argument)> = vec![];
        while let Some(arg) = reader.until().whitespace() {
            reader.move_to().next();
            if !arg.trim().is_empty() {
                let mut token = reader.token()?;
                if token.bound.contains().char(&chars::AT) {
                    Err(E::NestedFunction)?
                }
                arguments.push((token.id, Argument::String(Reader::unserialize(&arg))));
            }
        }
        if !reader.rest().trim().is_empty() {
            if reader.contains().char(&chars::AT) {
                Err(E::NestedFunction)?
            }
            let rest = reader.move_to().end();
            arguments.push((
                reader.token()?.id,
                Argument::String(Reader::unserialize(&rest)),
            ));
        }
        Ok(arguments)
    }
    pub fn add_args(&mut self, reader: &mut Reader) -> Result<(), E> {
        let mut arguments: Vec<(usize, Argument)> = vec![];
        loop {
            if reader.until().char(&[&chars::QUOTES]).is_some() {
                arguments = [
                    arguments,
                    Arguments::read_string_args(&mut reader.token()?.bound)?,
                ]
                .concat();
                if let Some(value_string) = ValueString::read(reader)? {
                    arguments.push((0, Argument::ValueString(value_string)));
                } else {
                    Err(E::NoStringEnd)?
                }
            } else {
                arguments = [arguments, Arguments::read_string_args(reader)?].concat();
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
