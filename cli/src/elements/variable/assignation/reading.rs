use crate::{
    elements::{Element, ElementRef, VariableAssignation},
    error::LinkedErr,
    reader::{chars, words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableAssignation> for VariableAssignation {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableAssignation>, LinkedErr<E>> {
        let close = reader.open_token(ElementRef::VariableAssignation);
        let global = reader.move_to().word(&[words::GLOBAL_VAR]).is_some();
        if let Some(variable) = Element::include(reader, &[ElementRef::VariableName])? {
            let rest = reader.rest().trim();
            if rest.starts_with(words::DO_ON)
                || rest.starts_with(words::CMP_TRUE)
                || !rest.starts_with(chars::EQUAL)
            {
                return Ok(None);
            }
            let _ = reader.move_to().char(&[&chars::EQUAL]);
            let assignation = Element::include(
                reader,
                &[
                    ElementRef::Block,
                    ElementRef::First,
                    ElementRef::Function,
                    ElementRef::If,
                    ElementRef::PatternString,
                    ElementRef::Values,
                    ElementRef::Comparing,
                    ElementRef::Command,
                    ElementRef::VariableName,
                    ElementRef::Integer,
                    ElementRef::Boolean,
                    ElementRef::Reference,
                    ElementRef::Compute,
                    ElementRef::Join,
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
