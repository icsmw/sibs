use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// Limits for generatinc tests with proptest
pub const FUNCTION: usize = 2;
pub const BLOCK: usize = 2;
pub const FIRST: usize = 1;
pub const COMMAND: usize = 10;
pub const COMPONENT: usize = 1;
pub const EACH: usize = 1;
pub const IF: usize = 1;
pub const META: usize = 10;
pub const OPTIONAL: usize = 1;
pub const REFERENCE: usize = 10;
pub const TASK: usize = 1;
pub const VALUE_STRING: usize = 1;
pub const VARIABLE_ASSIGNATION: usize = 10;
pub const VARIABLE_COMPARING: usize = 10;
pub const VARIABLE_DECLARATION: usize = 10;
pub const VARIABLE_NAME: usize = 10;
pub const VARIABLE_TYPE: usize = 10;
pub const VARIANTS: usize = 10;
pub const VALUES: usize = 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entity {
    Command,
    Component,
    Each,
    First,
    If,
    Function,
    Block,
    Meta,
    Optional,
    Reference,
    Task,
    ValueString,
    VariableAssignation,
    VariableComparing,
    VariableDeclaration,
    VariableName,
    VariableType,
    Variants,
    Values,
}

pub struct Permissions {
    pub func: bool,
    pub block: bool,
    pub first: bool,
    pub optional: bool,
    pub command: bool,
    pub each: bool,
    pub r#if: bool,
    pub component: bool,
    pub meta: bool,
    pub reference: bool,
    pub task: bool,
    pub value_string: bool,
    pub variable_assignation: bool,
    pub variable_comparing: bool,
    pub variable_declaration: bool,
    pub variable_name: bool,
    pub variable_type: bool,
    pub variants: bool,
    pub values: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub declarations: HashMap<String, Option<String>>,
    pub assignation: HashMap<String, Option<String>>,
    pub chain: Vec<Entity>,
}

impl Scope {
    pub fn add_declaration(&mut self, name: String) {
        self.declarations.insert(name, None);
    }
    pub fn add_assignation(&mut self, name: String) {
        self.assignation.insert(name, None);
    }
    pub fn include(&mut self, entity: Entity) {
        self.chain.push(entity);
    }
    pub fn exclude(&mut self, entity: Entity) {
        if let Some(last) = self.chain.last() {
            if last == &entity {
                let _ = self.chain.pop();
            }
        }
    }
    pub fn count_of(&self, entity: Entity) -> usize {
        self.chain.iter().filter(|en| en == &&entity).count()
    }
    pub fn permissions(&self) -> Permissions {
        Permissions {
            func: self.count_of(Entity::Function) < FUNCTION,
            first: self.count_of(Entity::First) < FIRST,
            block: self.count_of(Entity::Block) < BLOCK,
            optional: self.count_of(Entity::Optional) < OPTIONAL,
            command: self.count_of(Entity::Command) < COMMAND,
            r#if: self.count_of(Entity::If) < IF,
            each: self.count_of(Entity::Each) < EACH,
            component: self.count_of(Entity::Component) < COMPONENT,
            meta: self.count_of(Entity::Meta) < META,
            reference: self.count_of(Entity::Reference) < REFERENCE,
            value_string: self.count_of(Entity::ValueString) < VALUE_STRING,
            variable_assignation: self.count_of(Entity::VariableAssignation) < VARIABLE_ASSIGNATION,
            variable_comparing: self.count_of(Entity::VariableComparing) < VARIABLE_COMPARING,
            variable_declaration: self.count_of(Entity::VariableDeclaration) < VARIABLE_DECLARATION,
            variable_name: self.count_of(Entity::VariableName) < VARIABLE_NAME,
            variable_type: self.count_of(Entity::VariableType) < VARIABLE_TYPE,
            variants: self.count_of(Entity::Variants) < VARIANTS,
            task: self.count_of(Entity::Task) < TASK,
            values: self.count_of(Entity::Values) < VALUES,
        }
    }
}

pub type SharedScope = Arc<RwLock<Scope>>;
