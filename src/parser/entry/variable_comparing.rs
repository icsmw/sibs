use crate::parser::{
    chars,
    entry::{condition::Cmp, Function, Reader, Reading, ValueString, VariableName},
    words, E,
};

#[derive(Debug)]
pub struct VariableComparing {
    pub name: VariableName,
    pub cmp: Cmp,
    pub value: String,
}

impl Reading<VariableComparing> for VariableComparing {
    fn read(reader: &mut Reader) -> Result<Option<VariableComparing>, E> {
        reader.hold();
        if let Some(name) = VariableName::read(reader)? {
            if let Some(word) = reader.move_to_word(&[words::CMP_TRUE, words::CMP_FALSE])? {
                if reader.rest().trim().is_empty() {
                    Err(E::NoValueAfterComparing)
                } else {
                    Ok(Some(VariableComparing {
                        name,
                        cmp: if word == words::CMP_TRUE {
                            Cmp::Equal
                        } else {
                            Cmp::NotEqual
                        },
                        value: reader.rest().trim().to_string(),
                    }))
                }
            } else {
                reader.roll_back();
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

// #[cfg(test)]
// mod test {
//     use crate::parser::{
//         entry::{Reading, VariableAssignation},
//         Mapper, Reader, E,
//     };

//     #[test]
//     fn reading() -> Result<(), E> {
//         let mut mapper = Mapper::new();
//         let mut reader = Reader::new(
//             include_str!("./tests/variable_assignation.sibs").to_string(),
//             &mut mapper,
//             0,
//         );
//         while let Some(task) = VariableAssignation::read(&mut reader)? {
//             println!("{task:?}");
//         }

//         println!("{}", reader.rest().trim());
//         assert!(reader.rest().trim().is_empty());
//         Ok(())
//     }
// }
