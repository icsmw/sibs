use tokio_util::sync::CancellationToken;

use crate::{
    elements::{Component, ElTarget, Element, Gatekeeper},
    error::LinkedErr,
    inf::{operator, Context, Formation, FormationCursor, Operator, OperatorPinnedResult, Scope},
    reader::{chars, Reader, Reading, E},
};
use std::fmt;

const SELF: &str = "self";

#[derive(Debug, Clone)]
pub struct Call {
    pub token: usize,
}

impl Reading<Call> for Call {
    fn read(reader: &mut Reader) -> Result<Option<Self>, LinkedErr<E>> {
        Ok(None)
    }
}

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

impl Operator for Call {
    fn token(&self) -> usize {
        self.token
    }
    fn perform<'a>(
        &'a self,
        owner: Option<&'a Component>,
        components: &'a [Component],
        inputs: &'a [String],
        cx: Context,
        sc: Scope,
        token: CancellationToken,
    ) -> OperatorPinnedResult {
        Box::pin(async move { Ok(None) })
    }
}
