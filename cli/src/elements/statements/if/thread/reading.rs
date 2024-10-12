use crate::{
    elements::{Element, ElementRef, IfThread},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<IfThread> for IfThread {
    fn try_dissect(reader: &mut Reader) -> Result<Option<IfThread>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::IfThread);
        if reader.move_to().word(&[words::IF]).is_some() {
            let conditions = Element::include(
                reader,
                &[ElementRef::IfSubsequence, ElementRef::IfCondition],
            )?
            .ok_or(E::NoConditionForIfStatement.by_reader(reader))?;
            let block = Element::include(reader, &[ElementRef::Block])?
                .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
            Ok(Some(IfThread::If(Box::new(conditions), Box::new(block))))
        } else if reader.move_to().word(&[words::ELSE]).is_some() {
            let block = Element::include(reader, &[ElementRef::Block])?
                .ok_or(E::NoBlockForIfStatement.by_reader(reader))?;
            Ok(Some(IfThread::Else(Box::new(block))))
        } else {
            close(reader);
            Ok(None)
        }
    }
}

impl Dissect<IfThread, IfThread> for IfThread {}
