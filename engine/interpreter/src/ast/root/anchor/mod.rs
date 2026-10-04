use crate::*;

// Follow the owned import tree in source order, without entering executable bodies.
fn collect_imports<'a>(node: &'a LinkedNode, imports: &mut Vec<&'a LinkedNode>) {
    match node.get_node() {
        Node::Root(Root::GlobalsImport(_) | Root::EnvsImport(_)) => imports.push(node),
        Node::Root(Root::Anchor(_) | Root::Module(_))
        | Node::Declaration(
            Declaration::ModuleDeclaration(_) | Declaration::IncludeDeclaration(_),
        ) => {
            for child in node.childs() {
                collect_imports(child, imports);
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
        for node in &self.nodes {
            collect_imports(node, &mut imports);
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
