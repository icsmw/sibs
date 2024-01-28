use crate::{
    cli,
    inf::{
        any::AnyValue,
        context::Context,
        operator::{self, Operator, OperatorPinnedResult},
    },
    reader::{
        chars,
        entry::{Block, Component, Reading},
        words, Reader, E,
    },
};
use std::fmt;

#[derive(Debug)]
pub struct First {
    pub block: Block,
    pub token: usize,
}

impl Reading<First> for First {
    fn read(reader: &mut Reader) -> Result<Option<First>, E> {
        if reader.move_to().word(&[&words::FIRST]).is_some() {
            if reader
                .group()
                .between(&chars::OPEN_SQ_BRACKET, &chars::CLOSE_SQ_BRACKET)
                .is_some()
            {
                let mut token = reader.token()?;
                if reader.move_to().char(&[&chars::SEMICOLON]).is_none() {
                    Err(E::MissedSemicolon)
                } else {
                    let mut block = Block::read(&mut token.bound)?.ok_or(E::EmptyGroup)?;
                    block.use_as_first();
                    Ok(Some(First {
                        block,
                        token: token.id,
                    }))
                }
            } else {
                Err(E::NoGroup)
            }
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for First {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FIRST {};", self.block)
    }
}

impl Operator for First {
    fn process<'a>(
        &'a self,
        components: &'a [Component],
        args: &'a [String],
        cx: &'a mut Context,
    ) -> OperatorPinnedResult {
        Box::pin(async { self.block.process(components, args, cx).await })
    }
}

#[cfg(test)]
mod test_first {
    use crate::reader::{
        entry::{First, Reading},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("../tests/normal/first.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = First::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }

    #[test]
    fn error() -> Result<(), E> {
        let samples = include_str!("../tests/error/first.sibs").to_string();
        let samples = samples.split('\n').collect::<Vec<&str>>();
        let mut count = 0;
        for sample in samples.iter() {
            let mut reader = Reader::new(sample.to_string());
            assert!(First::read(&mut reader).is_err());
            count += 1;
        }
        assert_eq!(count, samples.len());
        Ok(())
    }
}
