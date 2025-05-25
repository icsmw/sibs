use crate::*;
use std::fmt::{Debug, Display};

pub(crate) fn resolve_conflicts<K: Display + Clone + PartialEq + ConflictResolver<K>>(
    mut results: Vec<(usize, LinkedNode, K)>,
    parser: &Parser,
) -> Result<Option<LinkedNode>, LinkedErr<E>> {
    let Some((n, (ppos, node, id))) = results
        .iter()
        .enumerate()
        .max_by_key(|(_, (_, n, ..))| n.get_md().link.exto().abs)
    else {
        return Ok(None);
    };
    let conflicted = results
        .iter()
        .filter(|(_, n, oid)| n.get_md().link.exto() == node.get_md().link.exto() && oid != id)
        .cloned()
        .collect::<Vec<(usize, LinkedNode, K)>>();
    if conflicted.is_empty() {
        parser.set_pos(*ppos);
        return Ok(Some(results.remove(n).1));
    };
    let (mut ppos, mut resolved_node, mut resolved_id) = (ppos, node.clone(), id.clone());
    let mut ignored = Vec::new();
    for (pos, cnode, id) in conflicted.iter() {
        if &resolved_id.resolve_conflict(id) == id && ignored.contains(id) {
            let err = E::NodesAreInConflict(
                results
                    .iter()
                    .filter(|(_, n, ..)| n.get_md().link.exto() == node.get_md().link.exto())
                    .map(|(.., id)| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            return Err(if let Some((_, node, _)) = results.first() {
                err.link(node)
            } else {
                err.link(node)
            });
        } else if &resolved_id.resolve_conflict(id) == id {
            ignored.push(resolved_id);
            resolved_id = id.clone();
            resolved_node = cnode.clone();
            ppos = pos;
        }
    }
    parser.set_pos(*ppos);
    Ok(Some(resolved_node))
}

pub trait ReadNode<T: Clone + Debug + Into<Node>>: Interest {
    fn read(parser: &Parser) -> Result<Option<T>, LinkedErr<E>>;
    fn read_as_linked(parser: &Parser) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        let mut md = Metadata::default();
        md.read_md_before(parser)?;
        let Some(tk_from) = parser.next() else {
            return Ok(None);
        };
        if !Self::intrested(&tk_from) {
            return Ok(None);
        }
        let Some(node) = Self::read(parser)? else {
            return Ok(None);
        };
        let Some(tk_before_md) = parser.current() else {
            return Err(LinkedErr::token(E::UnexpectedEmptyParser, &tk_from));
        };
        md.read_md_after(parser)?;
        let mut linked = LinkedNode::from_node(node.into());
        let Some(tk_after_md) = parser.current() else {
            return Err(LinkedErr::token(E::UnexpectedEmptyParser, &tk_from));
        };
        linked.get_mut_md().link.set_pos(&tk_from, &tk_before_md);
        linked.get_mut_md().link.set_expos(&tk_from, &tk_after_md);
        linked.get_mut_md().merge(md);
        Ok(Some(linked))
    }
}

pub(crate) trait TryRead<
    T: Clone + Debug + Into<Node>,
    K: Display + Clone + PartialEq + ConflictResolver<K>,
>
{
    fn try_read(parser: &Parser, id: K) -> Result<Option<LinkedNode>, LinkedErr<E>>;
    fn try_oneof(parser: &Parser, ids: &[K]) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        let mut results = Vec::new();
        if parser.next().is_none() {
            return Ok(None);
        };
        let reset = parser.pin();
        let from = parser.pos();
        for id in ids {
            let drop = parser.pin();
            if let Some(el) = Self::try_read(parser, id.clone())? {
                results.push((parser.pos(), el, id.to_owned()));
            }
            drop(parser);
        }
        reset(parser);
        match resolve_conflicts(results, parser)? {
            Some(node) => {
                parser.add_binding(from, parser.pos(), node.uuid());
                Ok(Some(node))
            }
            None => Ok(None),
        }
    }
}

pub trait TryReadOneOf<T, K> {
    fn try_oneof(parser: &Parser, ids: &[K]) -> Result<Option<T>, LinkedErr<E>>;
}

pub trait AsVec<T> {
    fn as_vec() -> Vec<T>;
}

pub(crate) trait ReadMetadata {
    fn read_md_before(&mut self, parser: &Parser) -> Result<(), LinkedErr<E>>;
    fn read_md_after(&mut self, parser: &Parser) -> Result<(), LinkedErr<E>>;
}
