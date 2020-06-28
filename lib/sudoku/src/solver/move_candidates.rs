use crate::prelude::Index;
use crate::game::Placement;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct MoveCandidates {
    pub index: Index,
    pub moves: HashSet<Placement>
}

impl MoveCandidates {
    pub fn new(index: Index) -> MoveCandidates {
        MoveCandidates { index, moves: HashSet::new() }
    }

    pub fn from_iter<I>(index: Index, moves: I) -> MoveCandidates
        where I: Iterator<Item=Placement> {
        MoveCandidates { index, moves: HashSet::from_iter(moves) }
    }

    pub fn add(&mut self, r#move: Placement) -> bool {
        self.moves.insert(r#move)
    }

    pub fn len(&self) -> usize { self.moves.len() }

    pub fn is_empty(&self) -> bool { self.moves.is_empty() }

    pub fn is_trivial(&self) -> bool {
        self.moves.len() == 1
    }

    pub fn is_branching(&self) -> bool {
        self.moves.len() > 1
    }

    pub fn eliminate(&mut self, r#move: &Placement) {
        self.moves.remove(r#move);
    }
}
