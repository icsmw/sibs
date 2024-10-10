use crate::{
    elements::{conditions::Cmb, Combination},
    error::LinkedErr,
    reader::{words, Dissect, Reader, TryDissect, E},
};

impl TryDissect<Combination> for Combination {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Combination>, LinkedErr<E>> {
        if reader.move_to().expression(&[words::AND]).is_some() {
            Ok(Some(Combination {
                cmb: Cmb::And,
                token: reader.token()?.id,
            }))
        } else if reader.move_to().expression(&[words::OR]).is_some() {
            Ok(Some(Combination {
                cmb: Cmb::Or,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Combination, Combination> for Combination {}
