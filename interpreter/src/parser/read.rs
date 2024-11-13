use crate::*;
use std::fmt::Display;

pub fn resolve_reading_conflicts<T: Clone, K: Display + Clone + PartialEq + ConflictResolver<K>>(
    mut results: Vec<(usize, T, K)>,
    parser: &mut Parser,
) -> Result<Option<T>, E> {
    if let Some((n, (pos, tk, id))) = results.iter().enumerate().max_by_key(|(_, (a, ..))| a) {
        let conflicted = results
            .iter()
            .filter(|(p, _, oid)| p == pos && oid != id)
            .cloned()
            .collect::<Vec<(usize, T, K)>>();
        if conflicted.is_empty() {
            parser.pos = *pos;
            Ok(Some(results.remove(n).1))
        } else if let (Some((_, c_tk, c_id)), true) = (conflicted.first(), conflicted.len() == 1) {
            parser.pos = *pos;
            if &id.resolve_conflict(c_id) == id {
                Ok(Some(tk.clone()))
            } else {
                Ok(Some(c_tk.clone()))
            }
        } else {
            Err(E::NodesAreInConflict(
                results
                    .iter()
                    .filter(|(p, ..)| p == pos)
                    .map(|(.., id)| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            ))
        }
    } else {
        Ok(None)
    }
}
pub trait ReadNode<T> {
    fn read(parser: &mut Parser) -> Result<Option<T>, E>;
}

pub trait TryRead<T: Clone, K: Display + Clone + PartialEq + ConflictResolver<K>> {
    fn try_read(parser: &mut Parser, id: K) -> Result<Option<T>, E>;
    fn try_oneof(parser: &mut Parser, ids: &[K]) -> Result<Option<T>, E> {
        let mut results = Vec::new();
        let reset = parser.pin();
        for id in ids {
            let drop = parser.pin();
            if let Some(el) = Self::try_read(parser, id.clone())? {
                results.push((parser.pos, el, id.to_owned()));
            }
            drop(parser);
        }
        reset(parser);
        resolve_reading_conflicts(results, parser)
    }
}

pub trait AsVec<T> {
    fn as_vec() -> Vec<T>;
}

pub trait Read<
    T: Clone + TryRead<T, K>,
    K: AsVec<K> + Interest + Display + Clone + PartialEq + ConflictResolver<K>,
>
{
    fn read(parser: &mut Parser) -> Result<Option<T>, E> {
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
