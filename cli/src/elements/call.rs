use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element, Gatekeeper},
    error::LinkedErr,
    inf::{
        operator, Context, Execute, ExecutePinnedResult, Formation, FormationCursor, Scope,
        TokenGetter, TryExecute,
    },
    reader::{chars, Dissect, Reader, TryDissect, E},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Call {
    pub token: usize,
}

impl TryDissect<Call> for Call {
    fn try_dissect(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        Ok(None)
    }
}

impl Dissect<Call, Call> for Call {}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "",)
    }
}

impl Formation for Call {
    fn format(&self, cursor: &mut FormationCursor) -> String {
        format!("{}{}", cursor.offset_as_string_if(&[ElTarget::Block]), self)
    }
}

impl TokenGetter for Call {
    fn token(&self) -> usize {
        self.token
    }
}

impl TryExecute for Call {
    fn try_execute<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> ExecutePinnedResult {
        Box::pin(async move { Ok(None) })
    }
}

impl Execute for Call {}
