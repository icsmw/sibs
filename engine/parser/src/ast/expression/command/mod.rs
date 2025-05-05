#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Command {
    fn intrested(token: &Token) -> bool {
        matches!(token.id(), KindId::Backtick)
    }
}

impl ReadNode<Command> for Command {
    fn read(parser: &Parser) -> Result<Option<Command>, LinkedErr<E>> {
        let Some(open) = parser.token() else {
            return Ok(None);
        };
        if !matches!(open.kind, Kind::Backtick) {
            return Ok(None);
        }
        let mut nodes = vec![CommandPart::Open(open.clone())];
        while let Some(token) = parser.token() {
            match token.id() {
                KindId::Literal => {
                    nodes.push(CommandPart::Literal(token.clone()));
                }
                KindId::LeftBrace => {
                    let node = LinkedNode::try_oneof(
                        parser,
                        &[
                            NodeTarget::Value(&[
                                ValueId::Number,
                                ValueId::Boolean,
                                ValueId::PrimitiveString,
                                ValueId::InterpolatedString,
                            ]),
                            NodeTarget::Statement(&[StatementId::If]),
                            NodeTarget::Expression(&[
                                ExpressionId::Variable,
                                ExpressionId::BinaryExpSeq,
                                ExpressionId::ComparisonSeq,
                                ExpressionId::FunctionCall,
                                ExpressionId::Command,
                            ]),
                        ],
                    )?
                    .ok_or_else(|| {
                        E::NotSupportedStringInjection(parser.to_string()).link_until_end(&parser)
                    })?;
                    let Some(next) = parser.token() else {
                        return Err(E::NoClosing(KindId::RightBrace).link(&node));
                    };
                    if !matches!(next.id(), KindId::RightBrace) {
                        return Err(E::NoClosing(KindId::RightBrace).link(&node));
                    }
                    nodes.push(CommandPart::Expression(node));
                }
                KindId::Backtick => {
                    nodes.push(CommandPart::Close(token.clone()));
                    break;
                }
                _ => {
                    return Err(
                        E::NotSupportedStringInjection(token.to_string()).link_with_token(&token)
                    )
                }
            }
        }
        if let (Some(CommandPart::Open(..)), Some(CommandPart::Close(..))) =
            (nodes.first(), nodes.last())
        {
            Ok(Some(Command {
                nodes,
                uuid: Uuid::new_v4(),
            }))
        } else {
            Err(E::InvalidString(open.to_string()).link_with_token(&open))
        }
    }
}
