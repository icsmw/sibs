use crate::{
    error::LinkedErr,
    inf::{Formation, FormationCursor},
    reader::{chars, words, Reader, Reading, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Comment {
    pub comment: String,
    pub token: usize,
}

impl Reading<Comment> for Comment {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let restore = reader.pin();
        if let Some(stop) = reader.move_to().expression(&[words::META, words::COMMENT]) {
            if stop == words::META {
                restore(reader);
                return Ok(None);
            }
            if reader.until().char(&[&chars::CARET]).is_none() {
                let _ = reader.move_to().end();
            } else {
                let _ = reader.move_to().next();
            }
            let token = reader.token()?;
            Ok(Some(Comment {
                comment: token.content.trim().to_owned(),
                token: token.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "// {}", self.comment)
    }
}

impl Formation for Comment {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        let mut collected = String::new();
        let mut lines: Vec<String> = Vec::new();
        for word in self.comment.split_whitespace() {
            collected = format!("{collected} {word}");
            if collected.len() >= cursor.max_len() {
                lines.push(collected);
                collected = String::new();
            }
        }
        if !collected.trim().is_empty() {
            lines.push(collected);
        }
        lines
            .iter()
            .map(|l| format!("{}// {l}", cursor.offset_as_string()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        elements::Task,
        error::LinkedErr,
        inf::tests::*,
        reader::{chars, Reading, E},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        runner(
            include_str!("../tests/reading/comments.sibs"),
            |mut src, mut reader| {
                while let Some(entity) = src.report_err_if(Task::read(&mut reader))? {
                    let _ = reader.move_to().char(&[&chars::SEMICOLON]);
                    for el in entity.block.elements.iter() {
                        assert_eq!(el.get_metadata().comments().len(), 2);
                    }
                }
                assert!(reader.rest().trim().is_empty());
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod proptest {

    use crate::elements::Comment;
    use proptest::prelude::*;

    impl Arbitrary for Comment {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            "[a-z][a-z0-9]*"
                .prop_map(String::from)
                .prop_map(|comment| Comment { comment, token: 0 })
                .boxed()
        }
    }
}
