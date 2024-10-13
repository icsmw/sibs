use crate::{
    elements::{Element, ElementId, Join, TokenGetter},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Join> for Join {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Join>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::Join);
        if reader.move_to().word(&[words::JOIN]).is_some() {
            let Some(Element::Values(elements, md)) =
                Element::include(reader, &[ElementId::Values])?
            else {
                return Err(E::NoJOINStatementBody.by_reader(reader));
            };
            if elements.elements.is_empty() {
                Err(E::NoJOINStatementBody.by_reader(reader))?;
            }
            for el in elements.elements.iter() {
                if !matches!(
                    el,
                    Element::Reference(..) | Element::Function(..) | Element::Command(..)
                ) {
                    Err(E::NotReferenceInJOIN.linked(&el.token()))?;
                }
            }
            let mut elements = Element::Values(elements, md);
            elements.drop_ppm(reader)?;
            Ok(Some(Join {
                elements: Box::new(elements),
                token: close(reader),
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Join, Join> for Join {}
