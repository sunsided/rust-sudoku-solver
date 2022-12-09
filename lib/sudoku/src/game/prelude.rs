use std::num::NonZeroU8;

pub type Index = u8;
pub type GroupId = u8;
pub type Coordinate = usize;
pub type Value = NonZeroU8;
pub type ValueOption = Option<Value>;

pub use super::indexbitset::IndexBitSet;
pub use super::valuebitset::ValueBitSet;

pub fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
