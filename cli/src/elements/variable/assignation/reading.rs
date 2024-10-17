use crate::{
    elements::{Element, ElementId, VariableAssignation},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableAssignation> for VariableAssignation {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableAssignation>, LinkedErr<E>> {
        let close = reader.open_token(ElementId::VariableAssignation);
        let global = reader.move_to().word(&[words::GLOBAL_VAR]).is_some();
        if let Some(variable) = Element::read(reader, &[ElementId::VariableName])? {
            let rest = reader.rest().trim();
            if rest.starts_with(words::DO_ON)
                || rest.starts_with(words::CMP_TRUE)
                || !rest.starts_with(chars::EQUAL)
            {
                return Ok(None);
            }
            let _ = reader.move_to().char(&[&chars::EQUAL]);
            let assignation = Element::read(
                reader,
                &[
                    ElementId::Block,
                    ElementId::First,
                    ElementId::Function,
                    ElementId::If,
                    ElementId::PatternString,
                    ElementId::Values,
                    ElementId::Comparing,
                    ElementId::Command,
                    ElementId::VariableName,
                    ElementId::Integer,
                    ElementId::Boolean,
                    ElementId::Reference,
                    ElementId::Compute,
                    ElementId::Join,
                ],
            )?
            .ok_or(E::FailToParseRightSideOfAssignation.by_reader(reader))?;
            Ok(Some(VariableAssignation {
                variable: Box::new(variable),
                global,
                assignation: Box::new(assignation),
                token: close(reader),
            }))
        } else if global {
            Err(E::InvalidUsageGlobalKeyword.by_reader(reader))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<VariableAssignation, VariableAssignation> for VariableAssignation {}
