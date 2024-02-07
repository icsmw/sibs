use proptest::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

// Limits for generatinc tests with proptest
pub const FUNCTION: usize = 2;
pub const BLOCK: usize = 2;
pub const FIRST: usize = 1;
pub const COMMAND: usize = usize::MAX;
pub const COMPONENT: usize = 1;
pub const EACH: usize = 1;
pub const IF: usize = 1;
pub const META: usize = 100;
pub const OPTIONAL: usize = 100;
pub const REFERENCE: usize = 100;
pub const TASK: usize = 1;
pub const VALUE_STRING: usize = 100;
pub const VARIABLE_ASSIGNATION: usize = usize::MAX;
pub const VARIABLE_COMPARING: usize = usize::MAX;
pub const VARIABLE_DECLARATION: usize = usize::MAX;
pub const VARIABLE_NAME: usize = usize::MAX;
pub const VARIABLE_TYPE: usize = usize::MAX;
pub const VARIANTS: usize = usize::MAX;

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
}

pub struct Permissions {
    pub func: bool,
    pub block: bool,
    pub first: bool,
    pub optional: bool,
    pub command: bool,
    pub each: bool,
    pub If: bool,
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
}

#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub declarations: HashMap<String, Option<String>>,
    pub assignation: HashMap<String, Option<String>>,
    pub chain: Vec<Entity>,
}

impl Scope {
    pub fn get_rnd_declaration_name(&self) -> BoxedStrategy<String> {
        prop::sample::select(self.declarations.keys().cloned().collect::<Vec<String>>()).boxed()
    }
    pub fn add_declaration(&mut self, name: String) {
        self.declarations.insert(name, None);
    }
    pub fn assign_declaration(&mut self, name: String, value: String) {
        self.declarations
            .entry(name)
            .and_modify(|v| {
                let _ = v.insert(value.clone());
            })
            .or_insert(Some(value));
    }
    pub fn add_assignation(&mut self, name: String) {
        self.assignation.insert(name, None);
    }
    pub fn assign_assignation(&mut self, name: String, value: String) {
        self.assignation
            .entry(name)
            .and_modify(|v| {
                let _ = v.insert(value.clone());
            })
            .or_insert(Some(value));
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
            If: self.count_of(Entity::If) < IF,
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
        }
    }
}

pub type SharedScope = Arc<RwLock<Scope>>;
