use crate::{
    elements::VariableVariants,
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableVariants> for VariableVariants {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableVariants>, LinkedErr<E>> {
        let content = reader
            .until()
            .char(&[&chars::SEMICOLON, &chars::COMMA])
            .map(|(content, _)| content)
            .unwrap_or_else(|| reader.move_to().end());
        Ok(Some(VariableVariants::new(content, reader.token()?.id)?))
    }
}

impl Dissect<VariableVariants, VariableVariants> for VariableVariants {}
