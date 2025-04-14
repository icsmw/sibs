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
    fn interpret(&self, rt: Runtime, cx: Context) -> RtPinnedResult<LinkedErr<E>> {
        match self {
            Declaration::ArgumentDeclaration(n) => n.interpret(rt, cx),
            Declaration::ClosureDeclaration(n) => n.interpret(rt, cx),
            Declaration::FunctionDeclaration(n) => n.interpret(rt, cx),
            Declaration::VariableDeclaration(n) => n.interpret(rt, cx),
            Declaration::VariableType(n) => n.interpret(rt, cx),
            Declaration::VariableTypeDeclaration(n) => n.interpret(rt, cx),
            Declaration::VariableVariants(n) => n.interpret(rt, cx),
            Declaration::VariableName(n) => n.interpret(rt, cx),
            Declaration::ModuleDeclaration(n) => n.interpret(rt, cx),
            Declaration::IncludeDeclaration(n) => n.interpret(rt, cx),
        }
    }
}
