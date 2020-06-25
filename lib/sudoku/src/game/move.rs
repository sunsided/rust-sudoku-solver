use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct MoveId {
    pub value: Value,
    pub index: Index
}

pub struct Move {
    pub value: Value,
    pub index: Index,
    pub x: Coordinate,
    pub y: Coordinate
}

impl Move {
    pub fn new(value: Value, index: Index, x: Coordinate, y: Coordinate) -> Move {
        Move { value, index, x, y}
    }

    pub fn id(&self) -> MoveId {
        MoveId { value: self.value, index: self.index }
    }
}

impl Hash for Move {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.index.hash(state);
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.index.eq(&other.index)
    }
}
impl Eq for Move {}
