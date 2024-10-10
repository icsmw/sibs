use crate::{
    elements::Integer,
    error::LinkedErr,
    reader::{Dissect, Reader, TryDissect, E},
};

impl TryDissect<Integer> for Integer {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Integer>, LinkedErr<E>> {
        reader.move_to().any();
        if let Some(value) = reader.move_to().none_numeric() {
            Ok(Some(Integer {
                value: value
                    .parse::<isize>()
                    .map_err(|e| E::IntegerParsingError(e.to_string()).by_reader(reader))?,
                token: reader.token()?.id,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Dissect<Integer, Integer> for Integer {}
