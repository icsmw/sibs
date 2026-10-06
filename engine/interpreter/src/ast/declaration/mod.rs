mod argument_declaration;
mod closure_declaration;
mod env_declaration;
mod envs_import;
mod function_declaration;
mod global_declaration;
mod globals_import;
mod include_declaration;
mod module_import;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;

impl Interpret for Declaration {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Declaration::GlobalDeclaration(n) => n.interpret(env),
            Declaration::EnvDeclaration(n) => n.interpret(env),
            Declaration::ArgumentDeclaration(n) => n.interpret(env),
            Declaration::ClosureDeclaration(n) => n.interpret(env),
            Declaration::FunctionDeclaration(n) => n.interpret(env),
            Declaration::VariableDeclaration(n) => n.interpret(env),
            Declaration::VariableType(n) => n.interpret(env),
            Declaration::VariableTypeDeclaration(n) => n.interpret(env),
            Declaration::VariableVariants(n) => n.interpret(env),
            Declaration::VariableName(n) => n.interpret(env),
            Declaration::GlobalsImport(n) => n.interpret(env),
            Declaration::EnvsImport(n) => n.interpret(env),
            Declaration::ModuleImport(n) => n.interpret(env),
            Declaration::IncludeDeclaration(n) => n.interpret(env),
        }
    }
}
