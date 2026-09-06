use crate::*;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub(crate) struct Candidate<T> {
    pos: usize,
    node: LinkedNode,
    id: T,
}

impl<T> Candidate<T> {
    pub fn new(pos: usize, node: LinkedNode, id: T) -> Self {
        Self { pos, node, id }
    }

    pub fn abs_position(&self) -> usize {
        self.node.get_md().link.exto().abs
    }

    pub fn is_same_position(&self, other: &Self) -> bool {
        self.node.get_md().link.exto() == other.node.get_md().link.exto()
    }
}

impl<T> Candidate<T>
where
    T: ConflictResolver<T>,
{
    pub fn resolve_conflict(&self, other: &Self) -> T {
        self.id.resolve_conflict(&other.id)
    }
}

pub(crate) struct CandidateList<T> {
    candidates: Vec<Candidate<T>>,
}

impl<T> Default for CandidateList<T> {
    fn default() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }
}

impl<T> CandidateList<T> {
    pub fn add(&mut self, pos: usize, node: LinkedNode, id: T) {
        self.candidates.push(Candidate::new(pos, node, id));
    }
}

impl<T> CandidateList<T>
where
    T: Display + Clone + PartialEq + ConflictResolver<T>,
{
    pub fn resolve_conflicts(
        &self,
        parser: &Parser,
    ) -> Result<Option<LinkedNode>, LinkedErr<E>> {
        let Some(candidate) = self
            .candidates
            .iter()
            .max_by_key(|candidate| candidate.abs_position())
        else {
            return Ok(None);
        };
        let conflicted_list = self
            .candidates
            .iter()
            .filter(|conflicted| {
                conflicted.is_same_position(candidate) && conflicted.id != candidate.id
            })
            .cloned()
            .collect::<Vec<Candidate<T>>>();
        if conflicted_list.is_empty() {
            parser.set_pos(candidate.pos);
            return Ok(Some(candidate.node.clone()));
        };
        let mut candidate = candidate.clone();
        let mut ignored = Vec::new();
        for conflicted in conflicted_list.iter() {
            if candidate.resolve_conflict(conflicted) == conflicted.id {
                if ignored.contains(&conflicted.id) {
                    let err = E::NodesAreInConflict(
                        self.candidates
                            .iter()
                            .filter(|other| candidate.is_same_position(*other))
                            .map(|conflicted| conflicted.id.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                    );
                    return Err(if let Some(first) = self.candidates.first() {
                        err.link(&first.node)
                    } else {
                        err.link(&candidate.node)
                    });
                } else {
                    ignored.push(candidate.id.clone());
                    candidate = conflicted.clone();
                }
            }
        }
        parser.set_pos(candidate.pos);
        Ok(Some(candidate.node))
    }
}

pub(crate) trait ConflictResolver<K> {
    fn resolve_conflict(&self, id: &K) -> K;
}
