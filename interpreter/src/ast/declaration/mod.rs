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
    fn interpret(&self, rt: Runtime) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.interpret(rt),
            Declaration::ClosureDeclaration(n) => n.interpret(rt),
            Declaration::FunctionDeclaration(n) => n.interpret(rt),
            Declaration::VariableDeclaration(n) => n.interpret(rt),
            Declaration::VariableType(n) => n.interpret(rt),
            Declaration::VariableTypeDeclaration(n) => n.interpret(rt),
            Declaration::VariableVariants(n) => n.interpret(rt),
            Declaration::VariableName(n) => n.interpret(rt),
            Declaration::ModuleDeclaration(n) => n.interpret(rt),
            Declaration::IncludeDeclaration(n) => n.interpret(rt),
        }
    }
}
