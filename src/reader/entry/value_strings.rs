use crate::{
    cli,
    inf::{any::AnyValue, context::Context, operator::Operator},
    reader::{
        chars,
        entry::{Function, Reader, Reading, VariableName},
        E,
    },
};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Injection {
    VariableName(String, VariableName),
    Function(String, Function),
}

impl Injection {
    pub fn hook(&self) -> &str {
        match self {
            Self::VariableName(hook, _) => &hook,
            Self::Function(hook, _) => &hook,
        }
    }
}

impl Operator for Injection {
    fn val<'a>(&'a mut self, cx: &'a mut Context) -> Result<&AnyValue, cli::error::E> {
        match self {
            Self::VariableName(_, v) => v.val(cx),
            Self::Function(_, v) => v.val(cx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ValueString {
    pub pattern: String,
    pub injections: Vec<Injection>,
    pub token: usize,
    val_ref: Option<Uuid>,
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
                let hook = token.content.clone();
                reader.move_to().next();
                if let Some(variable_name) = VariableName::read(&mut token.bound)? {
                    injections.push(Injection::VariableName(hook, variable_name));
                } else if let Some(func) = Function::read(&mut token.bound)? {
                    injections.push(Injection::Function(hook, func));
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
            val_ref: None,
        })
    }
}

impl fmt::Display for ValueString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.pattern,)
    }
}

impl Operator for ValueString {
    fn val<'a>(&'a mut self, cx: &'a mut Context) -> Result<&AnyValue, cli::error::E> {
        let uuid = if let Some(uuid) = self.val_ref.as_ref() {
            *uuid
        } else {
            let mut output = self.pattern.clone();
            for injection in self.injections.iter_mut() {
                let val = injection
                    .val(cx)?
                    .get_as_string()
                    .ok_or(cli::error::E::NoArguments)?;
                let hook = injection.hook();
                println!(">>>>>>>>>>>>>>>>>>>HOOK:__{hook}__");
                output = output.replace(hook, &val);
            }
            let uuid = Uuid::new_v4();
            cx.processed.insert(uuid.clone(), AnyValue::new(output));
            self.val_ref = Some(uuid);
            uuid
        };
        Ok(cx
            .processed
            .get(&uuid)
            .as_ref()
            .ok_or(cli::error::E::FailToExtractValue)?)
    }
}

#[cfg(test)]
mod test_value_string {
    use crate::reader::{
        entry::{Reading, ValueString},
        tests, Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/normal/value_string.sibs").to_string());
        let mut count = 0;
        while let Some(entity) = ValueString::read(&mut reader)? {
            assert_eq!(
                tests::trim(reader.recent()),
                tests::trim(&entity.to_string())
            );
            count += 1;
        }
        assert_eq!(count, 16);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
