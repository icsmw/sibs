mod argument_declaration;
mod closure_declaration;
mod function_declaration;
mod include_declaration;
mod module_declaration;
mod variable_declaration;
mod variable_name;
mod variable_type;
mod variable_type_declaration;
mod variable_variants;

use crate::*;

impl Interpret for Declaration {
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.interpret(env),
            Declaration::ClosureDeclaration(n) => n.interpret(env),
            Declaration::FunctionDeclaration(n) => n.interpret(env),
            Declaration::VariableDeclaration(n) => n.interpret(env),
            Declaration::VariableType(n) => n.interpret(env),
            Declaration::VariableTypeDeclaration(n) => n.interpret(env),
            Declaration::VariableVariants(n) => n.interpret(env),
            Declaration::VariableName(n) => n.interpret(env),
            Declaration::ModuleDeclaration(n) => n.interpret(env),
            Declaration::IncludeDeclaration(n) => n.interpret(env),
        }
    }
}
