use crate::{
    elements::Element,
    inf::{Atlas, Context, Journal, PrevValue, Scope, Value},
};
use tokio_util::sync::CancellationToken;

pub struct ExecuteContext<'a> {
    pub owner: Option<&'a Element>,
    pub components: &'a [Element],
    pub args: &'a [Value],
    pub prev: &'a Option<PrevValue>,
    pub cx: Context,
    pub sc: Scope,
    pub token: CancellationToken,
}

impl<'a> ExecuteContext<'a> {
    pub fn unbound(cx: Context, sc: Scope) -> Self {
        Self {
            owner: None,
            components: &[],
            args: &[],
            prev: &None,
            cx,
            sc,
            token: CancellationToken::new(),
        }
    }
    pub fn join(
        args: &(Option<Element>, Vec<Element>, Vec<Value>, Option<PrevValue>),
        props: (Context, Scope, CancellationToken),
    ) -> ExecuteContext<'_> {
        ExecuteContext {
            owner: args.0.as_ref(),
            components: &args.1,
            args: &args.2,
            prev: &args.3,
            cx: props.0,
            sc: props.1,
            token: props.2,
        }
    }
    pub fn is_aborting(&self) -> bool {
        self.cx.is_aborting()
    }
    pub fn journal(&self) -> &Journal {
        &self.cx.journal
    }
    pub fn atlas(&self) -> &Atlas {
        &self.cx.atlas
    }
    pub fn clone(&self) -> Self {
        Self {
            owner: self.owner,
            components: self.components,
            args: self.args,
            prev: self.prev,
            cx: self.cx.clone(),
            sc: self.sc.clone(),
            token: self.token.clone(),
        }
    }
    #[allow(clippy::type_complexity)]
    pub fn split(
        &self,
    ) -> (
        (Option<Element>, Vec<Element>, Vec<Value>, Option<PrevValue>),
        (Context, Scope, CancellationToken),
    ) {
        (
            (
                self.owner.cloned().clone(),
                self.components.to_vec(),
                self.args.to_vec(),
                self.prev.clone(),
            ),
            (self.cx.clone(), self.sc.clone(), self.token.clone()),
        )
    }
    pub fn owner(mut self, owner: Option<&'a Element>) -> Self {
        self.owner = owner;
        self
    }
    pub fn components(mut self, components: &'a [Element]) -> Self {
        self.components = components;
        self
    }
    pub fn prev(mut self, prev: &'a Option<PrevValue>) -> Self {
        self.prev = prev;
        self
    }
    pub fn args(mut self, args: &'a [Value]) -> Self {
        self.args = args;
        self
    }
    pub fn token(mut self, token: CancellationToken) -> Self {
        self.token = token;
        self
    }
    pub fn sc(mut self, sc: Scope) -> Self {
        self.sc = sc;
        self
    }
}
