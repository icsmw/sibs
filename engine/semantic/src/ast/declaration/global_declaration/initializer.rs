use crate::*;

/// Validate the expression tree allowed in a global or const initializer.
/// Only literals, arrays, string interpolation, arithmetic/comparison/logical
/// expressions and references to already visible constants or env entries are allowed.
/// Runs before registering the declaration; type checking is performed by its caller.
pub(super) fn validate(node: &LinkedNode, globals: &Globals) -> Result<(), LinkedErr<E>> {
    if !node.get_md().ppm.is_empty() {
        return Err(LinkedErr::from(
            E::InvalidGlobalInitializer("postfix operations are not allowed".into()),
            node,
        ));
    }
    // Enumerate every AST variant so new node kinds require reviewing these rules.
    match node.get_node() {
        Node::Expression(Expression::Variable(variable)) => {
            let symbol = globals.lookup(&variable.ident).ok_or_else(|| {
                LinkedErr::from(E::VariableIsNotDefined(variable.ident.clone()), node)
            })?;
            if symbol.kind == GlobalKind::Mutable {
                return Err(LinkedErr::from(
                    E::InvalidGlobalInitializer(format!(
                        "cannot read mutable global {}",
                        variable.ident
                    )),
                    node,
                ));
            }
        }
        Node::Value(
            Value::Number(_)
            | Value::Boolean(_)
            | Value::PrimitiveString(_)
            | Value::InterpolatedString(_)
            | Value::Array(_),
        )
        | Node::Expression(
            Expression::BinaryExp(_)
            | Expression::BinaryExpSeq(_)
            | Expression::BinaryExpGroup(_)
            | Expression::BinaryOp(_)
            | Expression::Comparison(_)
            | Expression::ComparisonSeq(_)
            | Expression::ComparisonGroup(_)
            | Expression::ComparisonOp(_)
            | Expression::LogicalOp(_),
        )
        | Node::Statement(Statement::AssignedValue(_)) => {}
        Node::Value(Value::Error(_) | Value::Closure(_))
        | Node::Expression(
            Expression::Call(_)
            | Expression::Accessor(_)
            | Expression::Range(_)
            | Expression::FunctionCall(_)
            | Expression::CompoundAssignments(_)
            | Expression::CompoundAssignmentsOp(_)
            | Expression::Command(_)
            | Expression::TaskCall(_),
        )
        | Node::Statement(
            Statement::Block(_)
            | Statement::Break(_)
            | Statement::Return(_)
            | Statement::Optional(_)
            | Statement::If(_)
            | Statement::For(_)
            | Statement::While(_)
            | Statement::Loop(_)
            | Statement::Assignation(_)
            | Statement::ArgumentAssignation(_)
            | Statement::ArgumentAssignedValue(_)
            | Statement::OneOf(_)
            | Statement::Join(_),
        )
        | Node::Declaration(
            Declaration::IncludeDeclaration(_)
            | Declaration::GlobalsImport(_)
            | Declaration::EnvsImport(_)
            | Declaration::ModuleImport(_)
            | Declaration::FunctionDeclaration(_)
            | Declaration::VariableDeclaration(_)
            | Declaration::GlobalDeclaration(_)
            | Declaration::EnvDeclaration(_)
            | Declaration::ArgumentDeclaration(_)
            | Declaration::VariableVariants(_)
            | Declaration::VariableType(_)
            | Declaration::VariableTypeDeclaration(_)
            | Declaration::VariableName(_)
            | Declaration::ClosureDeclaration(_),
        )
        | Node::Root(
            Root::Anchor(_)
            | Root::GlobalsModule(_)
            | Root::EnvsModule(_)
            | Root::Module(_)
            | Root::Component(_)
            | Root::Task(_),
        )
        | Node::ControlFlowModifier(
            ControlFlowModifier::Gatekeeper(_) | ControlFlowModifier::Skip(_),
        )
        | Node::Miscellaneous(
            Miscellaneous::Meta(_) | Miscellaneous::RootMeta(_) | Miscellaneous::Comment(_),
        ) => {
            return Err(LinkedErr::from(
                E::InvalidGlobalInitializer(node.get_node().to_string()),
                node,
            ));
        }
    }
    for child in node.childs() {
        validate(child, globals)?;
    }
    Ok(())
}
