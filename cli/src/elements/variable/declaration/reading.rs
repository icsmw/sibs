use crate::{
    elements::{Element, ElementId, VariableDeclaration},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableDeclaration> for VariableDeclaration {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableDeclaration>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::VariableDeclaration);
        let Some(variable) = Element::include(reader, &[ElementId::VariableName])? else {
            return Ok(None);
        };
        if reader.move_to().char(&[&chars::COLON]).is_none() {
            return Err(E::NoTypeDeclaration.by_reader(reader));
        }
        if let Some(declaration) = Element::include(
            reader,
            &[ElementId::VariableType, ElementId::VariableVariants],
        )? {
            Ok(Some(VariableDeclaration {
                variable: Box::new(variable),
                declaration: Box::new(declaration),
                token: close(reader),
            }))
        } else {
            Err(E::NoTypeDeclaration.by_reader(reader))
        }
    }
}

impl Dissect<VariableDeclaration, VariableDeclaration> for VariableDeclaration {}
