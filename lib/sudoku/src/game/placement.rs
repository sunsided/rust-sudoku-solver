use crate::game::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Placement {
    pub value: Value,
    pub index: Index,
}

impl Placement {
    pub fn new(value: Value, index: Index) -> Placement {
        Placement { value, index }
    }
}
