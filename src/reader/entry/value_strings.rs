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
    pub index: usize,
}

impl Reading<ValueString> for ValueString {
    fn read(reader: &mut Reader) -> Result<Option<ValueString>, E> {
        if reader.move_to_char(&[chars::QUOTES])?.is_some() {
            if let Some((pattern, _, _uuid)) = reader.read_until(&[chars::QUOTES], true, false)? {
                Ok(Some(ValueString::new(pattern, reader)?))
            } else {
                Err(E::NoStringEnd)
            }
        } else {
            Ok(None)
        }
    }
}

impl ValueString {
    pub fn new(pattern: String, parent: &mut Reader) -> Result<Self, E> {
        let mut reader = parent.inherit(pattern.clone());
        let mut injections: Vec<Injection> = vec![];
        let from = reader.pos;
        while reader.stop_on_char(chars::TYPE_OPEN, &[chars::QUOTES])? {
            if let Some((inner, _, _uuid)) = reader.read_until(&[chars::TYPE_CLOSE], true, false)? {
                let mut inner_reader = reader.inherit(inner);
                if let Some(variable_name) = VariableName::read(&mut inner_reader)? {
                    injections.push(Injection::VariableName(variable_name));
                } else if let Some(func) = Function::read(&mut inner_reader)? {
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
            index: reader.get_index_until_current(from),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::reader::{
        entry::{Reading, ValueString},
        Mapper, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut mapper = Mapper::new();
        let mut reader = Reader::new(
            include_str!("./tests/value_string.sibs").to_string(),
            &mut mapper,
            0,
        );
        while let Some(value_string) = ValueString::read(&mut reader)? {
            println!("{value_string:?}");
        }
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
