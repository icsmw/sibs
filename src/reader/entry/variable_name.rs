use crate::{
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Component, Reader, Reading},
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub token: usize,
}

impl Reading<VariableName> for VariableName {
    fn read(reader: &mut Reader) -> Result<Option<VariableName>, E> {
        if reader.move_to().char(&[&chars::DOLLAR]).is_some() {
            let content = reader
                .until()
                .char(&[&chars::COLON, &chars::WS, &chars::EQUAL, &chars::SEMICOLON])
                .map(|(content, _char)| content)
                .unwrap_or_else(|| reader.move_to().end());
            Ok(Some(VariableName::new(content, reader.token()?.id)?))
        } else {
            Ok(None)
        }
    }
}

impl VariableName {
    pub fn new(mut name: String, token: usize) -> Result<Self, E> {
        name = name.trim().to_string();
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
            || name.is_empty()
        {
            Err(E::InvalidVariableName)
        } else {
            Ok(Self { name, token })
        }
    }
}

impl Operator for VariableName {
    fn process<'a>(
        &'a self,
        _: Option<&'a Component>,
        _: &'a [Component],
        _: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async {
            let value = cx
                .get_var(&self.name)
                .await
                .ok_or(operator::E::VariableIsNotAssigned(self.name.to_owned()))?;
            Ok(value
                .get_as_string()
                .map(AnyValue::new)
                .or_else(|| value.get_as_strings().map(AnyValue::new)))
        })
    }
}

impl fmt::Display for VariableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}", self.name)
    }
}

#[cfg(test)]
mod reading {
    use crate::reader::{
        entry::{Reading, VariableName},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let samples = include_str!("../../tests/reading/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(VariableName::read(&mut reader).is_ok());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(VariableName::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::reader::entry::variable_name::VariableName;
    use proptest::prelude::*;

    impl Arbitrary for VariableName {
        type Parameters = Option<BoxedStrategy<String>>;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(name_strategy: Self::Parameters) -> Self::Strategy {
            if let Some(name_strategy) = name_strategy {
                name_strategy
            } else {
                "[a-z][a-z0-9]*".prop_map(String::from).boxed()
            }
            .prop_map(|name| VariableName { name, token: 0 })
            .boxed()
        }
    }
}
