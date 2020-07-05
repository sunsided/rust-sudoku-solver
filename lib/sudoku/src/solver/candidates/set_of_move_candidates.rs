use std::collections::{HashMap, HashSet};
use crate::prelude::*;
use crate::game::Placement;
use crate::solver::candidates::MoveCandidates;

#[derive(Debug, Clone)]
pub struct SetOfMoveCandidates {
    moves: HashMap<Index, HashSet<Placement>>
}

impl SetOfMoveCandidates {
    pub fn new() -> SetOfMoveCandidates { SetOfMoveCandidates { moves: HashMap::new() } }

    pub fn len(&self) -> usize { self.moves.len() }

    pub fn total_len(&self) -> usize {
        let mut size = 0usize;
        for (_, moves) in self.moves.iter() {
            size += moves.len();
        }
        size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, candidate: Placement) {
        self.moves.entry(candidate.index)
            .or_insert_with(HashSet::new)
            .insert(candidate);
    }

    pub fn remove_candidate(&mut self, candidate: &Placement) -> bool {
        self.moves.entry(candidate.index)
            .and_modify(move |x| { x.remove(candidate); });

        if self.moves.contains_key(&candidate.index) && self.moves[&candidate.index].is_empty() {
            self.moves.remove(&candidate.index);
        }

        self.moves.len() > 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=MoveCandidates> + 'a {
        self.moves.iter().map(|(key, value)| {
            MoveCandidates::from_iter(key.clone(), value.iter().map(|x| x.clone()))
        })
    }
}

impl std::ops::Index<Index> for SetOfMoveCandidates {
    type Output = HashSet<Placement>;

    fn index(&self, index: Index) -> &Self::Output {
        &self.moves[&index]
    }
}