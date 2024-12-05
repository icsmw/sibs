use crate::*;

impl Interest for ControlFlowModifierId {
    fn intrested(&self, token: &Token) -> bool {
        match self {
            Self::Gatekeeper => matches!(token.kind, Kind::Pound),
            Self::Skip => matches!(token.id(), KindId::Identifier),
        }
    }
}
