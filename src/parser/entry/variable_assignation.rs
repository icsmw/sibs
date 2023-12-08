use crate::parser::{
    chars,
    entry::{Function, Reader, Reading, VariableName},
    E,
};

#[derive(Debug)]
pub enum Assignation {
    Function(Function),
}
#[derive(Debug)]
pub struct VariableAssignation {
    pub name: VariableName,
    pub assignation: Assignation,
}

impl Reading<VariableAssignation> for VariableAssignation {
    fn read(reader: &mut Reader) -> Result<Option<VariableAssignation>, E> {
        reader.hold();
        if let Some(name) = VariableName::read(reader)? {
            if reader.move_to_char(chars::EQUAL)? {
                if let Some(chars::EQUAL) = reader.next_char() {
                    // This is condition
                    reader.roll_back();
                    return Ok(None);
                }
                if let Some(func) = Function::read(reader)? {
                    return Ok(Some(VariableAssignation::new(
                        name,
                        Assignation::Function(func),
                    )));
                }
            } else {
                Err(E::NoComparingOrAssignation)?
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }
}

impl VariableAssignation {
    pub fn new(name: VariableName, assignation: Assignation) -> Self {
        Self { name, assignation }
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
//         println!("{:?}", reader.mapper);
//         assert!(reader.rest().trim().is_empty());
//         Ok(())
//     }
// }
