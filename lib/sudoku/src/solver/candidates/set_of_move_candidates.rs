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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add_candidate(&mut self, candidate: Placement) {
        self.moves.entry(candidate.index)
            .or_insert_with(HashSet::new)
            .insert(candidate);
    }

    pub fn remove_candidate(&mut self, index: Index) -> Option<HashSet<Placement, RandomState>> {
        self.moves.remove(&index)
    }

    pub fn remove_candidates<I>(&mut self, indexes: I) -> ()
        where I: Iterator<Item=Index> {
        for index in indexes {
            self.moves.remove(&index);
        }
    }

    pub fn eliminate(&mut self, candidate: &Placement) {
        self.moves.entry(candidate.index)
            .and_modify(move |x| { x.remove(candidate); });
    }

    pub fn eliminate_many<I>(&mut self, candidates: I)
        where I: Iterator<Item=Placement> {
        for r#move in candidates {
            self.eliminate(&r#move);
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=MoveCandidates> + 'a {
        self.moves.iter().map(|(key, value)| {
            MoveCandidates::from_iter(key.clone(), value.iter().map(|x| x.clone()))
        })
    }

    pub fn trim(&mut self) {
        self.moves.retain(|_, value| value.len() > 0);
    }
}


