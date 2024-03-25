use crate::inf::{AnyValue, Context, Logs};

#[derive(Debug)]
pub struct Vars<'a> {
    bound: &'a mut Context,
}
impl<'a> Vars<'a> {
    pub fn new(bound: &'a mut Context) -> Self {
        Self { bound }
    }
    pub fn get(&self, name: &str) -> Option<&AnyValue> {
        self.bound
            .logger
            .log(format!("Reading variable: ${name};",));
        if !self.bound.variables.contains_key(name) {
            self.bound
                .logger
                .err(format!("Variable: ${name} doesn't exist;"));
        }
        self.bound.variables.get(name)
    }

    pub fn set(&mut self, name: String, value: AnyValue) {
        self.bound
            .logger
            .log(format!("Assignation: ${name} = {value}"));
        self.bound.variables.insert(name, value);
    }
}
