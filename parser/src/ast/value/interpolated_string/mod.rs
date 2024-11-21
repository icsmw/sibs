mod link;
#[cfg(test)]
mod proptests;

use crate::*;
use asttree::*;
use lexer::{Kind, StringPart};

impl ReadNode<InterpolatedString> for InterpolatedString {
    fn read(parser: &mut Parser) -> Result<Option<InterpolatedString>, LinkedErr<E>> {
        let Some(tk) = parser.token().cloned() else {
            return Ok(None);
        };
        let Kind::InterpolatedString(parts) = &tk.kind else {
            return Ok(None);
        };
        let mut nodes = Vec::new();
        for part in parts.clone().into_iter() {
            match part {
                StringPart::Open(tk) => {
                    nodes.push(InterpolatedStringPart::Open(tk));
                }
                StringPart::Close(tk) => {
                    nodes.push(InterpolatedStringPart::Close(tk));
                }
                StringPart::Literal(s) => {
                    nodes.push(InterpolatedStringPart::Literal(s));
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
                    let mut inner = Parser::new(tks, &parser.src);
                    if inner.is_done() {
                        return Err(E::EmptyStringExpression.link_between(&l_br, &r_br));
                    }
                    let Some(node) = Node::try_oneof(
                        &mut inner,
                        &[
                            NodeReadTarget::Value(&[
                                ValueId::Number,
                                ValueId::Boolean,
                                ValueId::PrimitiveString,
                                ValueId::InterpolatedString,
                            ]),
                            NodeReadTarget::Statement(&[StatementId::If]),
                            NodeReadTarget::Expression(&[
                                ExpressionId::Variable,
                                ExpressionId::BinaryExpSeq,
                                ExpressionId::ComparisonSeq,
                                ExpressionId::FunctionCall,
                                ExpressionId::Command,
                            ]),
                        ],
                    )?
                    else {
                        return Err(E::NotSupportedStringInjection(inner.to_string())
                            .link_until_end(&inner));
                    };
                    if !inner.is_done() {
                        return Err(E::UnrecognizedCode(inner.to_string()).link_until_end(&inner));
                    }
                    nodes.push(InterpolatedStringPart::Expression(node));
                }
            };
        }
        if let (Some(InterpolatedStringPart::Open(..)), Some(InterpolatedStringPart::Close(..))) =
            (nodes.first(), nodes.last())
        {
            Ok(Some(InterpolatedString { nodes, token: tk }))
        } else {
            Err(E::InvalidString(tk.to_string()).link_with_token(&tk))
        }
    }
}
