use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct Move {
    pub value: Value,
    pub index: Index
}

impl Move {
    pub fn new(value: Value, index: Index) -> Move {
        Move { value, index }
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move { value: self.value, index: self.index }
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
