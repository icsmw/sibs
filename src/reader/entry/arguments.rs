use crate::reader::{
    chars,
    entry::{PatternString, Reader, Reading, SimpleString, VariableName},
    E,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Argument {
    SimpleString(SimpleString),
    PatternString(PatternString),
    VariableName(VariableName),
    Arguments(Arguments),
}

impl Argument {
    pub fn token(&self) -> usize {
        match self {
            Self::SimpleString(v) => v.token,
            Self::PatternString(v) => v.token,
            Self::VariableName(v) => v.token,
            Self::Arguments(v) => v.token,
        }
    }
}

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::SimpleString(v) => Reader::serialize(&v.value),
                Self::PatternString(v) => v.to_string(),
                Self::VariableName(v) => v.to_string(),
                Self::Arguments(v) => format!("[{v}]"),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Arguments {
    pub args: Vec<Argument>,
    pub token: usize,
}

impl Reading<Arguments> for Arguments {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        let mut args = Arguments {
            args: vec![],
            token: 0,
        };
        let close = reader.open_token();
        let mut close_group = reader.open_token();
        while reader
            .group()
            .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
            .is_some()
        {
            let mut token = reader.token()?;
            let mut group = Arguments {
                args: vec![],
                token: close_group(reader),
            };
            group.add_args(&mut token.bound)?;
            args.args.push(Argument::Arguments(group));
            close_group = reader.open_token();
        }
        if !reader.move_to().end().is_empty() {
            args.add_args(&mut reader.token()?.bound)?;
        }
        if args.is_empty() {
            Ok(None)
        } else {
            args.token = reader.token()?.id;
            args.token = close(reader);
            Ok(Some(args))
        }
    }
}

impl Arguments {
    pub fn get(&self, index: usize) -> Option<&Argument> {
        self.args.get(index).map(|arg| arg)
    }
    pub fn read_string_args(reader: &mut Reader) -> Result<Vec<Argument>, E> {
        let mut arguments: Vec<Argument> = vec![];
        while let Some(arg) = reader.until().whitespace() {
            reader.move_to().next();
            if !arg.trim().is_empty() {
                let mut token = reader.token()?;
                if token.bound.contains().char(&chars::AT) {
                    Err(E::NestedFunction)?
                }
                if let Some(variable) = VariableName::read(&mut token.bound)? {
                    arguments.push(Argument::VariableName(variable));
                } else if let Some(value_string) = PatternString::read(&mut token.bound)? {
                    arguments.push(Argument::PatternString(value_string));
                } else {
                    arguments.push(Argument::SimpleString(SimpleString {
                        value: Reader::unserialize(&arg),
                        token: token.id,
                    }));
                }
            }
        }
        if !reader.rest().trim().is_empty() {
            if reader.contains().char(&chars::AT) {
                Err(E::NestedFunction)?
            }
            if let Some(variable) = VariableName::read(reader)? {
                arguments.push(Argument::VariableName(variable));
            } else {
                let rest = reader.move_to().end();
                arguments.push(Argument::SimpleString(SimpleString {
                    value: Reader::unserialize(&rest),
                    token: reader.token()?.id,
                }));
            }
        }
        Ok(arguments)
    }
    pub fn add_args(&mut self, reader: &mut Reader) -> Result<(), E> {
        let mut arguments: Vec<Argument> = vec![];
        loop {
            if reader.until().char(&[&chars::QUOTES]).is_some() {
                arguments = [
                    arguments,
                    Arguments::read_string_args(&mut reader.token()?.bound)?,
                ]
                .concat();
                if let Some(value_string) = PatternString::read(reader)? {
                    arguments.push(Argument::PatternString(value_string));
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
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod proptest {
    use crate::{
        inf::tests::*,
        reader::entry::{
            arguments::{Argument, Arguments},
            pattern_string::PatternString,
            simple_string::SimpleString,
            variable_name::VariableName,
        },
    };
    use proptest::prelude::*;
    impl Arbitrary for Argument {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            let permissions = scope.read().unwrap().permissions();
            let mut allowed = vec!["[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|v| Argument::SimpleString(SimpleString { value: v, token: 0 }))
                .boxed()];
            if permissions.variable_name {
                allowed.push(
                    VariableName::arbitrary()
                        .prop_map(Argument::VariableName)
                        .boxed(),
                );
            }
            if permissions.value_string {
                allowed.push(
                    PatternString::arbitrary_with(scope.clone())
                        .prop_map(Argument::PatternString)
                        .boxed(),
                );
            }
            prop::strategy::Union::new(allowed).boxed()
        }
    }
    impl Arbitrary for Arguments {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(scope: Self::Parameters) -> Self::Strategy {
            prop::collection::vec(Argument::arbitrary_with(scope.clone()), 0..=5)
                .prop_map(|args| Arguments { args, token: 0 })
                .boxed()
        }
    }
}
