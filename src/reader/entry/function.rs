use crate::reader::{
    chars,
    entry::{Argument, Arguments, Reading},
    words, Reader, E,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub tolerance: bool,
    pub name: String,
    pub args: Option<Arguments>,
    pub token: usize,
}

impl Reading<Function> for Function {
    fn read(reader: &mut Reader) -> Result<Option<Self>, E> {
        reader.state().set();
        if reader.move_to().char(&[&chars::AT]).is_some() {
            let (name, ends_with) = reader
                .until()
                .char(&[
                    &chars::CARET,
                    &chars::QUESTION,
                    &chars::SEMICOLON,
                    &chars::WS,
                ])
                .map(|(str, char)| (str, Some(char)))
                .unwrap_or_else(|| (reader.move_to().end(), None));
            if matches!(ends_with, Some(chars::SEMICOLON)) {
                reader.move_to().next();
                return Ok(Some(Self::new(
                    reader.token()?.id,
                    &mut reader.token()?.bound,
                    name,
                    false,
                )?));
            }
            if ends_with.is_none() {
                return Ok(Some(Self::new(reader.token()?.id, reader, name, false)?));
            }
            reader.trim();
            let stop_on = reader
                .until()
                .char(&[&chars::REDIRECT, &chars::SEMICOLON])
                .map(|(_, stop_on)| Some(stop_on))
                .unwrap_or_else(|| {
                    reader.move_to().end();
                    None
                });
            let mut token = reader.token()?;
            if token.bound.contains().word(&[&words::DO_ON]) {
                let _ = reader.state().restore();
                return Ok(None);
            }
            reader.move_to().next();
            if matches!(stop_on, Some(chars::REDIRECT)) {
                let arg_func = Self::new(
                    token.id,
                    &mut token.bound,
                    name,
                    matches!(ends_with, Some(chars::QUESTION)),
                )?;
                if let Some(mut parent_func) = Function::read(reader)? {
                    parent_func.add_fn_arg(arg_func);
                    Ok(Some(parent_func))
                } else {
                    Err(E::NoDestFunction)
                }
            } else {
                Ok(Some(Self::new(
                    token.id,
                    &mut token.bound,
                    name,
                    matches!(ends_with, Some(chars::QUESTION)),
                )?))
            }
        } else {
            Ok(None)
        }
    }
}

impl Function {
    pub fn new(
        token: usize,
        reader: &mut Reader,
        name: String,
        tolerance: bool,
    ) -> Result<Self, E> {
        Ok(Self {
            token,
            name,
            tolerance,
            args: Arguments::read(reader)?,
        })
    }
    pub fn add_fn_arg(&mut self, fn_arg: Function) {
        if let Some(args) = self.args.as_mut() {
            args.add_fn_arg(fn_arg);
        } else {
            self.args = Some(Arguments {
                args: vec![(0, Argument::Function(fn_arg))],
                token: 0,
            });
        }
    }
}

#[cfg(test)]
mod test_functions {
    use crate::reader::{
        entry::{Function, Reading},
        Reader, E,
    };

    #[test]
    fn reading() -> Result<(), E> {
        let mut reader = Reader::new(include_str!("./tests/function.sibs").to_string());
        let mut count = 0;
        while let Some(func) = Function::read(&mut reader)? {
            println!("{func:?}");
            count += 1;
        }
        assert_eq!(count, 13);
        assert!(reader.rest().trim().is_empty());
        Ok(())
    }
}
