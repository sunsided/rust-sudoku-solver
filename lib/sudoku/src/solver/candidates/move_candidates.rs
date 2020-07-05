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
    pub fn from_iter<I>(index: Index, moves: I) -> MoveCandidates
        where I: Iterator<Item=Placement> {
        MoveCandidates { index, moves: HashSet::from_iter(moves) }
    }

    pub fn is_trivial(&self) -> bool {
        self.moves.len() == 1
    }
}
