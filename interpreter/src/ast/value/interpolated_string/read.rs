use lexer::{Kind, StringPart};

use crate::*;

impl ReadNode<InterpolatedString> for InterpolatedString {
    fn read(parser: &mut Parser, _nodes: &Nodes) -> Result<Option<InterpolatedString>, E> {
        let Some(tk) = parser.token() else {
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
                        ));
                    };
                    if !matches!(f.kind, Kind::LeftBrace) || !matches!(l.kind, Kind::RightBrace) {
                        return Err(E::NotSupportedStringInjection(
                            tks.iter()
                                .map(|t| t.to_string())
                                .collect::<Vec<String>>()
                                .join(" "),
                        ));
                    }
                    let _ = tks.remove(0);
                    let _ = tks.remove(tks.len() - 1);
                    let mut inner = Parser::new(tks);
                    if inner.is_done() {
                        return Err(E::EmptyStringExpression);
                    }
                    let Some(node) = Node::try_oneof(
                        &mut inner,
                        &Nodes::empty(),
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
                        return Err(E::NotSupportedStringInjection(inner.to_string()));
                    };
                    if !inner.is_done() {
                        return Err(E::UnrecognizedCode(inner.to_string()));
                    }
                    nodes.push(InterpolatedStringPart::Expression(node));
                }
            };
        }
        if let (Some(InterpolatedStringPart::Open(..)), Some(InterpolatedStringPart::Close(..))) =
            (nodes.first(), nodes.last())
        {
            Ok(Some(InterpolatedString { nodes }))
        } else {
            Err(E::InvalidString(tk.to_string()))
        }
    }
}
