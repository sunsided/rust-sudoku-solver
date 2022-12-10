use crate::game::Placement;
use crate::prelude::{Index, Value};
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct MoveCandidates {
    pub index: Index,
    pub moves: HashSet<Placement>,
}

impl MoveCandidates {
    pub fn from_iter<I>(index: Index, moves: I) -> MoveCandidates
    where
        I: Iterator<Item = Placement>,
    {
        MoveCandidates {
            index,
            moves: HashSet::from_iter(moves),
        }
    }

    pub fn is_trivial(&self) -> bool {
        self.moves.len() == 1
    }

    /// Gets all possible placeable values for this move
    /// as a [`HashSet`].
    pub fn value_hashset(&self) -> HashSet<Value> {
        self.moves.iter().map(|p| p.value).collect()
    }

    /// Gets the number of possible value placements.
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn empty(&self) -> bool {
        self.moves.is_empty()
    }
}
