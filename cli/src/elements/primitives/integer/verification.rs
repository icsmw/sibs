use crate::{
    elements::{Element, Integer},
    inf::{
        Context, ExpectedResult, LinkingResult, PrevValueExpectation, TryExpectedValueType,
        ValueRef, VerificationResult,
    },
};

impl TryExpectedValueType for Integer {
    fn try_verification<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> VerificationResult<'a> {
        Box::pin(async move { Ok(()) })
    }

    fn try_linking<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> LinkingResult<'a> {
        Box::pin(async move { Ok(()) })
    }
    fn try_expected<'a>(
        &'a self,
        _owner: &'a Element,
        _components: &'a [Element],
        _prev: &'a Option<PrevValueExpectation>,
        _cx: &'a Context,
    ) -> ExpectedResult<'a> {
        Box::pin(async move {
            Ok(ValueRef::OneOf(vec![
                ValueRef::u8,
                ValueRef::u16,
                ValueRef::u32,
                ValueRef::u64,
                ValueRef::u128,
                ValueRef::usize,
                ValueRef::i8,
                ValueRef::i16,
                ValueRef::i32,
                ValueRef::i64,
                ValueRef::i128,
                ValueRef::isize,
            ]))
        })
    }
}
