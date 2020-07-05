use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use crate::prelude::*;
use crate::game::Placement;
use crate::solver::candidates::MoveCandidates;

#[derive(Debug, Clone)]
pub struct SetOfMoveCandidates {
    moves: HashMap<Index, HashSet<Placement>>
}

impl SetOfMoveCandidates {
    pub fn new() -> SetOfMoveCandidates { SetOfMoveCandidates { moves: HashMap::new() } }

    pub fn from_iter<I>(candidates: I) -> SetOfMoveCandidates
        where I: Iterator<Item=MoveCandidates> {
        let mut moves = HashMap::<Index, HashSet<Placement>>::new();
        for candidate in candidates.into_iter() {
            moves.entry(candidate.index)
                .or_insert_with(HashSet::new)
                .extend(candidate.moves);
        }
        SetOfMoveCandidates { moves }
    }

    pub fn len(&self) -> usize { self.moves.len() }

    pub fn total_len(&self) -> usize {
        let mut size = 0usize;
        for (idx, moves) in self.moves.iter() {
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

    pub fn add_many<I>(&mut self, candidates: I)
        where I: Iterator<Item=Placement> {
        for c in candidates {
            self.add(c);
        }
    }

    pub fn remove_index(&mut self, index: Index) -> bool {
        self.moves.remove(&index);
        self.moves.len() > 0
    }

    pub fn remove_indexes<I>(&mut self, indexes: I) -> bool
        where I: Iterator<Item=Index> {
        for index in indexes {
            self.moves.remove(&index);
        }

        self.moves.len() > 0
    }

    pub fn remove_candidate(&mut self, candidate: &Placement) -> bool {
        self.moves.entry(candidate.index)
            .and_modify(move |x| { x.remove(candidate); });

        if self.moves.contains_key(&candidate.index) && self.moves[&candidate.index].is_empty() {
            self.moves.remove(&candidate.index);
        }

        self.moves.len() > 0
    }

    pub fn remove_candidates<I>(&mut self, candidates: I) -> bool
        where I: Iterator<Item=Placement> {
        for r#move in candidates {
            self.remove_candidate(&r#move);
        }

        self.moves.len() > 0
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=MoveCandidates> + 'a {
        self.moves.iter().map(|(key, value)| {
            MoveCandidates::from_iter(key.clone(), value.iter().map(|x| x.clone()))
        })
    }

    pub fn into_iter(self) -> impl Iterator<Item=MoveCandidates> {
        self.moves.into_iter().map(|(key, value)| {
            MoveCandidates::from_iter(key, value.into_iter())
        })
    }

    pub fn trim(&mut self) {
        self.moves.retain(|_, value| value.len() > 0);
    }

    pub fn contains_key(&self, index: &Index) -> bool {
        self.moves.contains_key(index)
    }
}

impl std::ops::Index<Index> for SetOfMoveCandidates {
    type Output = HashSet<Placement>;

    fn index(&self, index: Index) -> &Self::Output {
        &self.moves[&index]
    }
}