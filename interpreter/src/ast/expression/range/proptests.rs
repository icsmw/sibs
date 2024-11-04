// use crate::*;

// use lexer::{gens::kind, Kind, KindId, Token};
// use proptest::prelude::*;
// use uuid::Uuid;

// impl Arbitrary for Range {
//     type Parameters = ();

//     type Strategy = BoxedStrategy<Self>;

//     fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
//         kind(KindId::Identifier)
//             .prop_filter_map("single ident", |knds| {
//                 if let Some(Kind::Identifier(ident)) = knds.first() {
//                     Some((ident.to_owned(), Kind::Identifier(ident.to_owned())))
//                 } else {
//                     None
//                 }
//             })
//             .prop_map(|(ident, knd)| Variable {
//                 ident,
//                 token: Token::by_pos(knd, &Uuid::new_v4(), 0, 0),
//             })
//             .boxed()
//     }
// }
