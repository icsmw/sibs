use crate::*;
use diagnostics::*;
use fmt::Debug;
use std::fmt::Display;

pub fn resolve_reading_conflicts<
    T: Clone + Debug,
    K: Display + Clone + PartialEq + ConflictResolver<K>,
>(
    mut results: Vec<(usize, T, K)>,
    parser: &mut Parser,
) -> Result<Option<T>, LinkedErr<E>>
where
    for<'a> SrcLink: From<&'a T>,
{
    if let Some((n, (pos, node, id))) = results.iter().enumerate().max_by_key(|(_, (a, ..))| a) {
        let conflicted = results
            .iter()
            .filter(|(p, _, oid)| p == pos && oid != id)
            .cloned()
            .collect::<Vec<(usize, T, K)>>();
        if conflicted.is_empty() {
            parser.pos = *pos;
            Ok(Some(results.remove(n).1))
        } else {
            let (mut resolved_node, mut resolved_id) = (node.clone(), id.clone());
            let mut ignored = Vec::new();
            for (_, node, id) in conflicted.iter() {
                if &resolved_id.resolve_conflict(id) == id {
                    if ignored.contains(&resolved_id) {
                        return Err(E::NodesAreInConflict(
                            results
                                .iter()
                                .filter(|(p, ..)| p == pos)
                                .map(|(.., id)| id.to_string())
                                .collect::<Vec<String>>()
                                .join(", "),
                        )
                        .link(
                            &results
                                .first()
                                .map(|(_, n, _)| n.into())
                                .unwrap_or_default(),
                        ));
                    }
                    ignored.push(resolved_id.clone());
                    resolved_id = id.clone();
                    resolved_node = node.clone();
                }
            }
            parser.pos = *pos;
            Ok(Some(resolved_node))
        }
    } else {
        Ok(None)
    }
}
pub trait ReadNode<T> {
    fn read(parser: &mut Parser) -> Result<Option<T>, LinkedErr<E>>;
}

pub trait TryRead<T: Clone + Debug, K: Display + Clone + PartialEq + ConflictResolver<K>>
where
    for<'a> SrcLink: From<&'a T>,
{
    fn try_read(parser: &mut Parser, id: K) -> Result<Option<T>, LinkedErr<E>>;
    fn try_oneof(parser: &mut Parser, ids: &[K]) -> Result<Option<T>, LinkedErr<E>> {
        let mut results = Vec::new();
        let reset = parser.pin();
        for id in ids {
            let drop = parser.pin();
            if let Some(el) = Self::try_read(parser, id.clone())? {
                // TODO: parser pos should not be considered because it might be read with PPM,
                // which should be ignored to resolve conflicts correctly.
                // Each node should safe start pos and finish pos. SrcLink should depend on it, but not on
                // calculated values.
                results.push((parser.pos, el, id.to_owned()));
            }
            drop(parser);
        }
        reset(parser);
        // if results.len() > 1 {
        //     println!(
        //         ">>>>>>>>>>>>>>>>>>>>>>>: {}",
        //         results
        //             .iter()
        //             .map(|(n, t, k)| format!("\n{n}: {k}\n{t:?}"))
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        // }
        resolve_reading_conflicts(results, parser)
    }
}

pub trait TryReadOneOf<T, K> {
    fn try_oneof(parser: &mut Parser, ids: &[K]) -> Result<Option<T>, LinkedErr<E>>;
}

pub trait AsVec<T> {
    fn as_vec() -> Vec<T>;
}

pub trait Read<
    T: Clone + Debug + TryRead<T, K>,
    K: AsVec<K> + Interest + Display + Clone + PartialEq + ConflictResolver<K>,
> where
    for<'a> SrcLink: From<&'a T>,
{
    fn read(parser: &mut Parser) -> Result<Option<T>, LinkedErr<E>> {
        let mut results = Vec::new();
        let reset = parser.pin();
        let Some(token) = parser.token() else {
            return Ok(None);
        };
        let intrested = K::as_vec()
            .into_iter()
            .filter(|k| k.intrested(token))
            .collect::<Vec<K>>();
        for id in intrested {
            let drop = parser.pin();
            if let Some(el) = T::try_read(parser, id.clone())? {
                results.push((parser.pos, el, id));
            }
            drop(parser);
        }
        reset(parser);
        resolve_reading_conflicts(results, parser)
    }
}

pub trait ReadMetadata {
    fn read_md_before(&mut self, parser: &mut Parser) -> Result<(), LinkedErr<E>>;
    fn read_md_after(&mut self, parser: &mut Parser) -> Result<(), LinkedErr<E>>;
}
