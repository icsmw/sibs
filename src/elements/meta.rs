use crate::{
    error::LinkedErr,
    inf::{term, Formation, FormationCursor, Term},
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Meta {
    pub inner: Vec<String>,
    pub token: usize,
}

impl Meta {
    pub fn as_string(&self) -> String {
        self.inner.join("\n")
    }
    pub fn as_lines(&self) -> Vec<&str> {
        self.inner.iter().map(|s| s.as_str()).collect()
    }
}

impl Reading<Meta> for Meta {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let mut inner: Vec<String> = vec![];
        while reader.move_to().expression(&[words::META]).is_some() {
            if let Some((line, _)) = reader.until().char(&[&chars::CARET]) {
                inner.push(line.trim().to_string());
            } else {
                Err(E::NoMetaContent.by_reader(reader))?
            }
        }
        if inner.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Meta {
                inner,
                token: reader.token()?.id,
            }))
        }
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|v| format!("/// {v}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Formation for Meta {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        self.inner
            .iter()
            .map(|v| format!("{}/// {v}", cursor.offset_as_string()))
            .collect::<Vec<String>>()
            .join("\n")
            .to_string()
    }
}

impl term::Display for Meta {
    fn display(&self, term: &mut Term) {
        term.print_fmt(&self.as_lines());
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::meta::Meta;
    use proptest::prelude::*;

    impl Arbitrary for Meta {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop::collection::vec("[a-z][a-z0-9]*".prop_map(String::from), 1..=10)
                .prop_map(|inner| Meta { inner, token: 0 })
                .boxed()
        }
    }
}
