use crate::reader::{
    chars,
    entry::{Reader, Reading, ValueString, VariableName},
    E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Argument {
    String(String),
    ValueString(ValueString),
    VariableName(VariableName),
    Arguments(Arguments),
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(v) => Reader::serialize(v),
                Self::ValueString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::Arguments(v) => format!("[{v}]"),
            }
        )
    }
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
            let mut token = reader.token()?;
            let mut group = Arguments {
                args: vec![],
                token: token.id,
            };
            group.add_args(&mut token.bound)?;
            args.args.push((token.id, Argument::Arguments(group)));
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
    pub fn get(&self, index: usize) -> Option<&Argument> {
        self.args.get(index).map(|(_, arg)| arg)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Argument> {
        self.args.get_mut(index).map(|(_, arg)| arg)
    }
    pub fn read_string_args(reader: &mut Reader) -> Result<Vec<(usize, Argument)>, E> {
        let mut arguments: Vec<(usize, Argument)> = vec![];
        while let Some(arg) = reader.until().whitespace() {
            reader.move_to().next();
            if !arg.trim().is_empty() {
                let mut token = reader.token()?;
                if token.bound.contains().char(&chars::AT) {
                    Err(E::NestedFunction)?
                }
                if let Some(variable) = VariableName::read(&mut token.bound)? {
                    arguments.push((token.id, Argument::VariableName(variable)));
                } else if let Some(value_string) = ValueString::read(&mut token.bound)? {
                    arguments.push((token.id, Argument::ValueString(value_string)));
                } else {
                    arguments.push((token.id, Argument::String(Reader::unserialize(&arg))));
                }
            }
        }
        if !reader.rest().trim().is_empty() {
            if reader.contains().char(&chars::AT) {
                Err(E::NestedFunction)?
            }
            if let Some(variable) = VariableName::read(reader)? {
                arguments.push((reader.token()?.id, Argument::VariableName(variable)));
            } else {
                let rest = reader.move_to().end();
                arguments.push((
                    reader.token()?.id,
                    Argument::String(Reader::unserialize(&rest)),
                ));
            }
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
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|(_, arg)| arg.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
