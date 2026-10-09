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

impl Lint for Declaration {
    fn lint(&self, md: &Metadata, ctx: &mut LintContext) -> Result<(), LinkedErr<E>> {
        match self {
            Declaration::GlobalDeclaration(n) => n.lint(md, ctx),
            Declaration::EnvDeclaration(n) => n.lint(md, ctx),
            Declaration::ArgumentDeclaration(n) => n.lint(md, ctx),
            Declaration::ClosureDeclaration(n) => n.lint(md, ctx),
            Declaration::FunctionDeclaration(n) => n.lint(md, ctx),
            Declaration::VariableDeclaration(n) => n.lint(md, ctx),
            Declaration::VariableType(n) => n.lint(md, ctx),
            Declaration::VariableTypeDeclaration(n) => n.lint(md, ctx),
            Declaration::VariableVariants(n) => n.lint(md, ctx),
            Declaration::VariableName(n) => n.lint(md, ctx),
            Declaration::GlobalsImport(n) => n.lint(md, ctx),
            Declaration::EnvsImport(n) => n.lint(md, ctx),
            Declaration::ModuleImport(n) => n.lint(md, ctx),
            Declaration::IncludeDeclaration(n) => n.lint(md, ctx),
        }
    }
}
