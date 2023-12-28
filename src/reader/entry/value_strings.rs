use crate::reader::{
    chars,
    entry::{Function, Reader, Reading, VariableName},
    E,
};

#[derive(Debug, Clone)]
pub enum Injection {
    VariableName(VariableName),
    Function(Function),
}

#[derive(Debug, Clone)]
pub struct ValueString {
    pub pattern: String,
    pub injections: Vec<Injection>,
    pub token: usize,
}

impl Reading<ValueString> for ValueString {
    fn read(reader: &mut Reader) -> Result<Option<ValueString>, E> {
        if let Some(inner) = reader.group().closed(&chars::QUOTES) {
            let mut token = reader.token()?;
            Ok(Some(ValueString::new(inner, &mut token.bound)?))
        } else {
            Ok(None)
        }
    }
}

impl ValueString {
    pub fn new(pattern: String, reader: &mut Reader) -> Result<Self, E> {
        let mut injections: Vec<Injection> = vec![];
        let token = reader.token()?.id;
        while reader.seek_to().char(&chars::TYPE_OPEN) {
            reader.move_to().next();
            if reader.until().char(&[&chars::TYPE_CLOSE]).is_some() {
                let mut token = reader.token()?;
                reader.move_to().next();
                if let Some(variable_name) = VariableName::read(&mut token.bound)? {
                    injections.push(Injection::VariableName(variable_name));
                } else if let Some(func) = Function::read(&mut token.bound)? {
                    injections.push(Injection::Function(func));
                } else {
                    Err(E::NoVariableReference)?
                }
            } else {
                Err(E::NoInjectionClose)?
            }
        }
        Ok(ValueString {
            pattern,
            injections,
            token,
        })
    }
}

#[cfg(test)]
mod test_value_string {
    use crate::reader::{
        entry::{Reading, ValueString},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/value_string.sibs").to_string());
        let mut count = 0;
        while let Some(value_string) = ValueString::read(&mut reader)? {
            println!("{value_string:?}");
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
