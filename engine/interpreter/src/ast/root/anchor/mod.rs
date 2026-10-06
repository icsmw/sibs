use crate::*;
use std::collections::HashSet;

// Follow the owned import tree in source order, without entering executable bodies.
fn collect_imports<'a>(
    node: &'a LinkedNode,
    imports: &mut Vec<&'a LinkedNode>,
    modules: &mut HashSet<Uuid>,
) {
    match node.get_node() {
        Node::Declaration(Declaration::GlobalsImport(_) | Declaration::EnvsImport(_)) => {
            imports.push(node)
        }
        Node::Declaration(Declaration::ModuleImport(module)) => {
            if modules.insert(module.body.source) {
                for child in &module.body.nodes {
                    collect_imports(child, imports, modules);
                }
            }
        }
        Node::Root(Root::Anchor(_) | Root::Module(_))
        | Node::Declaration(Declaration::IncludeDeclaration(_)) => {
            for child in node.childs() {
                collect_imports(child, imports, modules);
            }
        }
        _ => {}
    }
}

impl Interpret for Anchor {
    #[boxed]
    fn interpret(&self, env: InterpreterEnvironment) -> RtPinnedResult<'_, LinkedErr<E>> {
        let InterpreterEnvironment { rt, .. } = env.clone();
        let rt_params = rt
            .get_rt_parameters()
            .await
            .map_err(|err| LinkedErr::from(err, self))?;
        let Some(component) = self.get_component(&rt_params.component) else {
            return Err(LinkedErr::from(E::CompNotFound(rt_params.component), self));
        };
        let mut imports = Vec::new();
        let mut modules = HashSet::new();
        for node in &self.nodes {
            collect_imports(node, &mut imports, &mut modules);
        }
        for import in imports
            .iter()
            .filter(|node| node.extract::<EnvsImport>().is_some())
        {
            import.interpret(env.clone()).await?;
        }
        for import in imports
            .iter()
            .filter(|node| node.extract::<GlobalsImport>().is_some())
        {
            import.interpret(env.clone()).await?;
        }
        component.interpret(env).await
    }
}
