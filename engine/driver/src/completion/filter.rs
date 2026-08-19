use crate::*;
use runtime::DeterminedTy;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Filter {
    Variables(Option<Ty>),
    FunctionArgument(Option<Ty>),
    FunctionCall(Option<Ty>),
    All(Option<Ty>),
}

pub fn get_filter(
    loc: &Location,
    before_token: &Token,
    before_node: &LinkedNode,
    scx: &SemanticCx,
    ty_scope: &TyScope,
) -> Option<Filter> {
    match &loc.cursor {
        Cursor::Token(token, ..) => {
            if token.id() == KindId::Dot {
                let ty = find_ty_by_node(before_node, ty_scope, scx);
                return Some(Filter::FunctionArgument(ty));
            }
            match before_token.kind {
                Kind::Identifier(..) => Some(Filter::All(None)),
                Kind::Dot => {
                    let ty = find_ty_by_node(before_node, ty_scope, scx);
                    match token.id() {
                        KindId::Identifier | KindId::Dot => Some(Filter::FunctionArgument(ty)),
                        _ => None,
                    }
                }
                Kind::Number(..) => match token.id() {
                    KindId::Dot => Some(Filter::FunctionArgument(Some(Ty::Determined(
                        DeterminedTy::Num,
                    )))),
                    _ => None,
                },
                Kind::String(..) | Kind::SingleQuote => match token.id() {
                    KindId::Dot => Some(Filter::FunctionArgument(Some(Ty::Determined(
                        DeterminedTy::Str,
                    )))),
                    _ => None,
                },
                Kind::Keyword(Keyword::Let) => None,
                Kind::Keyword(Keyword::True) | Kind::Keyword(Keyword::False) => match token.id() {
                    KindId::Dot => Some(Filter::FunctionArgument(Some(Ty::Determined(
                        DeterminedTy::Bool,
                    )))),
                    _ => None,
                },
                Kind::Keyword(Keyword::If) => match token.id() {
                    KindId::Identifier => Some(Filter::All(None)),
                    _ => None,
                },
                Kind::LeftParen => None,
                Kind::LeftBrace => None,
                Kind::Colon => None,
                Kind::Comma => None,
                Kind::Semicolon => Some(Filter::All(None)),
                Kind::Equals => {
                    let ty = find_ty_by_node(before_node, ty_scope, scx);
                    match token.id() {
                        KindId::Identifier => Some(Filter::All(ty)),
                        _ => Some(Filter::All(None)),
                    }
                }
                Kind::Plus
                | Kind::Minus
                | Kind::Star
                | Kind::Slash
                | Kind::Less
                | Kind::Greater
                | Kind::LessEqual
                | Kind::GreaterEqual => Some(Filter::All(Some(Ty::Determined(DeterminedTy::Num)))),
                _ => None,
            }
        }
        Cursor::Path(_path, ..) => Some(Filter::All(None)),
    }
}

fn find_ty_by_node(node: &LinkedNode, scope: &TyScope, scx: &SemanticCx) -> Option<Ty> {
    match node.get_node() {
        Node::Expression(Expression::Variable(node)) => {
            scx.find_linked_ty(&node.uuid).or_else(|| {
                scope
                    .lookup(&node.ident)
                    .and_then(|entity| entity.ty())
                    .cloned()
            })
        }
        Node::Declaration(Declaration::VariableName(node)) => scope
            .lookup(&node.ident)
            .and_then(|entity| entity.ty())
            .cloned(),
        Node::Declaration(Declaration::VariableType(node)) => scx.table.get(&node.uuid),
        // Node::Declaration(Declaration::VariableVariants(..))
        Node::Declaration(Declaration::VariableTypeDeclaration(node)) => node
            .types
            .first()
            .and_then(|n| find_ty_by_node(n, scope, scx)),
        _ => None,
    }
}
