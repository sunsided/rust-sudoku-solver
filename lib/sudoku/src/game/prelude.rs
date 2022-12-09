use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::num::NonZeroU8;

pub type Index = usize;
pub type Coordinate = usize;
pub type Value = NonZeroU8;
pub type ValueOption = Option<Value>;

pub type IndexSet = HashSet<Index, RandomState>;

pub fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
