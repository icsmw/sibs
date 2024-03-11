use crate::{
    entry::Component,
    error::LinkedErr,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct VariableName {
    pub name: String,
    pub token: usize,
}

impl Reading<VariableName> for VariableName {
    fn read(reader: &mut Reader) -> Result<Option<VariableName>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token();
        if reader.move_to().char(&[&chars::DOLLAR]).is_some() {
            let content = reader
                .until()
                .char(&[&chars::COLON, &chars::WS, &chars::EQUAL, &chars::SEMICOLON])
                .map(|(content, _char)| content)
                .unwrap_or_else(|| reader.move_to().end());
            Ok(Some(VariableName::new(content, close(reader))?))
        } else {
            Ok(None)
        }
    }
}

impl VariableName {
    pub fn new(mut name: String, token: usize) -> Result<Self, LinkedErr<E>> {
        name = name.trim().to_string();
        if !Reader::is_ascii_alphabetic_and_alphanumeric(&name, &[&chars::UNDERSCORE, &chars::DASH])
            || name.is_empty()
        {
            Err(E::InvalidVariableName.linked(&token))
        } else {
            Ok(Self { name, token })
        }
    }
}

impl Operator for VariableName {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        _: Option<&'a Component>,
        _: &'a [Component],
        _: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async {
            let value = cx
                .get_var(&self.name)
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
    use crate::{
        entry::VariableName,
        error::LinkedErr,
        reader::{Reader, Reading, E},
    };

    #[test]
    fn reading() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/reading/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            VariableName::read(&mut reader)?.unwrap();
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn tokens() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/reading/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            let variable_name = VariableName::read(&mut reader)?.unwrap();
            let fragment = reader.get_fragment(&reader.token()?.id)?.content;
            assert_eq!(format!("${}", variable_name.name), fragment);
            assert_eq!(fragment, variable_name.to_string());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), LinkedErr<E>> {
        let samples = include_str!("../../tests/error/variable_name.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::unbound(sample.to_string());
            assert!(VariableName::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::entry::variable::VariableName;
    use proptest::prelude::*;

    impl Arbitrary for VariableName {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|name| VariableName {
                    name: if name.is_empty() {
                        "min".to_owned()
                    } else {
                        name
                    },
                    token: 0,
                })
                .boxed()
        }
    }
}
