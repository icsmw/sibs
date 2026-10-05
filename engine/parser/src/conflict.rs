use crate::*;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub(crate) struct Candidate<T> {
    pos: usize,
    node: LinkedNode,
    id: T,
    bindings: Vec<Binding>,
}

impl<T> Candidate<T> {
    pub fn new(pos: usize, node: LinkedNode, id: T, bindings: Vec<Binding>) -> Self {
        Self {
            pos,
            node,
            id,
            bindings,
        }
    }

    pub fn abs_position(&self) -> usize {
        self.node.get_md().link.exto().abs
    }

    pub fn is_same_position(&self, other: &Self) -> bool {
        self.node.get_md().link.exto() == other.node.get_md().link.exto()
    }
    pub fn node(self) -> LinkedNode {
        self.node
    }
    pub fn as_node(&self) -> &LinkedNode {
        &self.node
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn uuid(&self) -> &Uuid {
        self.node.uuid()
    }
    pub fn bind(&mut self, parser: &Parser, from: usize, to: usize) -> Result<(), E> {
        let mut bindings = parser
            .bindings
            .try_borrow_mut()
            .map_err(|err| E::EarlyFlushCall(err.to_string()))?;
        bindings.append(&mut self.bindings);
        bindings.add(*self.uuid(), from, to);
        Ok(())
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
    pub fn add(&mut self, pos: usize, node: LinkedNode, id: T, bindings: Vec<Binding>) {
        self.candidates
            .push(Candidate::new(pos, node, id, bindings));
    }
}

impl<T> CandidateList<T>
where
    T: Display + Clone + PartialEq + ConflictResolver<T>,
{
    pub fn resolve_conflicts(mut self) -> Result<Option<Candidate<T>>, LinkedErr<E>> {
        let Some((mut winner, candidate)) = self
            .candidates
            .iter()
            .enumerate()
            .max_by_key(|(_, candidate)| candidate.abs_position())
        else {
            return Ok(None);
        };
        let conflicted = self
            .candidates
            .iter()
            .enumerate()
            .filter(|(_, other)| other.is_same_position(candidate) && other.id != candidate.id)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let mut ignored = Vec::new();
        for index in conflicted {
            let candidate = &self.candidates[winner];
            let other = &self.candidates[index];
            if candidate.resolve_conflict(other) == other.id {
                if ignored.contains(&other.id) {
                    let err = E::NodesAreInConflict(
                        self.candidates
                            .iter()
                            .filter(|other| candidate.is_same_position(other))
                            .map(|other| other.id.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                    return Err(err.link(&self.candidates[0].node));
                }
                ignored.push(candidate.id.clone());
                winner = index;
            }
        }
        // Move the selected AST and its journal; all losing journals are dropped.
        Ok(Some(self.candidates.swap_remove(winner)))
    }
}

pub(crate) trait ConflictResolver<K> {
    fn resolve_conflict(&self, id: &K) -> K;
}
