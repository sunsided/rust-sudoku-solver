use crate::game::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Ord, PartialOrd)]
pub struct Placement {
    pub index: Index,
    pub value: Value,
}

impl Placement {
    pub fn new(value: Value, index: Index) -> Placement {
        Placement { value, index }
    }
}
