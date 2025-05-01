#[cfg(test)]
mod proptests;

use crate::*;

impl Interest for Command {
    fn intrested(token: &Token) -> bool {
        matches!(token.id(), KindId::Command)
    }
}

impl ReadNode<Command> for Command {
    fn read(parser: &Parser) -> Result<Option<Command>, LinkedErr<E>> {
        let Some(tk) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::Command(parts) = &tk.kind else {
            return Ok(None);
        };
        let mut nodes = Vec::new();
        for part in parts.clone().into_iter() {
            match part {
                StringPart::Open(tk) => {
                    nodes.push(CommandPart::Open(tk));
                }
                StringPart::Close(tk) => {
                    nodes.push(CommandPart::Close(tk));
                }
                StringPart::Literal(s) => {
                    nodes.push(CommandPart::Literal(s));
                }
                StringPart::Expression(mut tks) => {
                    let (Some(f), Some(l)) = (tks.first(), tks.last()) else {
                        return Err(E::NotSupportedStringInjection(
                            tks.iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                        )
                        .link_with_token(&tk));
                    };
                    if !matches!(f.kind, Kind::LeftBrace) || !matches!(l.kind, Kind::RightBrace) {
                        return Err(E::NotSupportedStringInjection(
                            tks.iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                        )
                        .link_between(f, l));
                    }
                    let l_br = tks.remove(0);
                    let r_br = tks.remove(tks.len() - 1);
                    let mut inner = parser.inherit(tks);
                    if inner.is_done() {
                        return Err(E::EmptyStringExpression.link_between(&l_br, &r_br));
                    }
                    let node = LinkedNode::try_oneof(
                        &mut inner,
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
                        E::NotSupportedStringInjection(inner.to_string()).link_until_end(&inner)
                    })?;
                    if !inner.is_done() {
                        return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
                    }
                    nodes.push(CommandPart::Expression(node));
                }
            };
        }
        if let (Some(CommandPart::Open(..)), Some(CommandPart::Close(..))) =
            (nodes.first(), nodes.last())
        {
            Ok(Some(Command {
                nodes,
                token: tk,
                uuid: Uuid::new_v4(),
            }))
        } else {
            Err(E::InvalidString(tk.to_string()).link_with_token(&tk))
        }
    }
}
