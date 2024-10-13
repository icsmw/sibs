use crate::{
    elements::{ElementId, VariableName},
    error::LinkedErr,
    reader::{chars, Dissect, Reader, TryDissect, E},
};

impl TryDissect<VariableName> for VariableName {
    fn try_dissect(reader: &mut Reader) -> Result<Option<VariableName>, LinkedErr<E>> {
        reader.move_to().any();
        let close = reader.open_token(ElementId::VariableName);
        if reader.move_to().char(&[&chars::DOLLAR]).is_some() {
            let content = reader
                .until()
                .char(&[
                    &chars::COLON,
                    &chars::WS,
                    &chars::EQUAL,
                    &chars::SEMICOLON,
                    &chars::COMMA,
                    &chars::OPEN_SQ_BRACKET,
                    &chars::DOT,
                    &chars::INC,
                    &chars::DEC,
                    &chars::EXCLAMATION,
                    &chars::CMP_LBIG,
                    &chars::CMP_RBIG,
                ])
                .map(|(content, _char)| content)
                .unwrap_or_else(|| reader.move_to().end());
            Ok(Some(VariableName::new(content, close(reader))?))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<VariableName, VariableName> for VariableName {}
