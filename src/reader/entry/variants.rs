use crate::{
    inf::term,
    reader::{
        chars,
        entry::{Reader, Reading},
        E,
    },
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Variants {
    pub values: Vec<String>,
    pub token: usize,
}

impl Reading<Variants> for Variants {
    fn read(reader: &mut Reader) -> Result<Option<Variants>, E> {
        let content = reader
            .until()
            .char(&[&chars::SEMICOLON])
            .map(|(content, _)| content)
            .unwrap_or_else(|| reader.move_to().end());
        Ok(Some(Variants::new(content, reader.token()?.id)?))
    }
}

impl Variants {
    pub fn new(input: String, token: usize) -> Result<Self, E> {
        let mut values: Vec<String> = vec![];
        for value in input.split('|') {
            let value = value.trim();
            if !value.is_ascii() {
                Err(E::NotAsciiValue(value.to_string()))?;
            }
            if chars::has_reserved(value) {
                Err(E::UsingReservedChars)?
            }
            if value.is_empty() {
                Err(E::EmptyValue)?;
            }
            values.push(value.to_string());
        }
        if values.is_empty() {
            Err(E::NoVariableValues)?;
        }
        Ok(Variants { values, token })
    }

    pub fn parse(&self, value: String) -> Option<String> {
        if self.values.contains(&value) {
            Some(value)
        } else {
            None
        }
    }
}

impl fmt::Display for Variants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values.join(" | "))
    }
}

impl term::Display for Variants {
    fn to_string(&self) -> String {
        format!("[{}]", self.values.join(" | "))
    }
}

#[cfg(test)]
mod test_values {
    use crate::reader::{
        entry::{Reading, Variants},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let samples = include_str!("../../tests/reading/variants.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Variants::read(&mut reader).is_ok());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../../tests/error/variants.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(Variants::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}

#[cfg(test)]
mod proptest {
    use crate::{inf::tests::*, reader::entry::variants::Variants};
    use proptest::prelude::*;

    impl Arbitrary for Variants {
        type Parameters = SharedScope;
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_scope: Self::Parameters) -> Self::Strategy {
            prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 0..=10)
                .prop_map(|values| Variants { values, token: 0 })
                .boxed()
        }
    }
}
