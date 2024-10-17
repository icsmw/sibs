use crate::{
    elements::{Element, ElementId, If, IfThread},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<If> for If {
    fn try_dissect(reader: &mut Reader) -> Result<Option<If>, LinkedErr<E>> {
        let mut threads: Vec<IfThread> = Vec::new();
        let close = reader.open_token(ElementId::If);
        while !reader.rest().trim().is_empty() {
            if reader.move_to().word(&[words::IF]).is_some() {
                let conditions =
                    Element::read(reader, &[ElementId::IfSubsequence, ElementId::IfCondition])?
                        .ok_or(E::NoConditionForIfStatement.by_reader(reader))?;
                let block = Element::read(reader, &[ElementId::Block])?
                    .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
                threads.push(IfThread::If(Box::new(conditions), Box::new(block)));
            } else if reader.move_to().word(&[words::ELSE]).is_some() {
                if threads.is_empty() {
                    Err(E::NoMainBlockForIfStatement.by_reader(reader))?;
                }
                let block = Element::read(reader, &[ElementId::Block])?
                    .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
                threads.push(IfThread::Else(Box::new(block)));
            } else {
                break;
            }
        }
        if threads.is_empty() {
            Ok(None)
        } else {
            Ok(Some(If {
                threads,
                token: close(reader),
            }))
        }
    }
}

impl Dissect<If, If> for If {}
