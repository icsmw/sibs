use crate::{
    elements::Comment,
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Comment> for Comment {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        let Some(stop) = reader.move_to().expression(&[words::META, words::COMMENT]) else {
            return Ok(None);
        };
        if stop == words::META {
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
    }
}

impl Dissect<Comment, Comment> for Comment {}
