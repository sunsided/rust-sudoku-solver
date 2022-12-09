use crate::prelude::Index;

/// A simple bitset for storing regular Sudoku-sized (i.e., up to 81) index values.
///
/// ## Technical Notes
/// Practically this implementation allows for storing up to 127 different indexes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IndexBitSet {
    /// We anticipate at most 81 fields on a standard Sudoku game.
    /// We use a 128-bit type here to directly encode the field values,
    /// even though this wastes 47 bits.
    state: u128,
}

impl IndexBitSet {
    #[inline]
    pub const fn with_value(mut self, index: Index) -> Self {
        debug_assert!(index < 81);
        let value = index as u128;
        self.state |= 1u128 << value;
        self
    }

    #[inline]
    pub fn insert(&mut self, index: Index) -> &mut Self {
        debug_assert!(index < 81);
        let value = index as u128;
        self.state |= 1u128 << value;
        self
    }

    #[inline]
    pub const fn without_value(mut self, index: Index) -> Self {
        debug_assert!(index < 81);
        let value = index as u128;
        self.state &= !(1u128 << value);
        self
    }

    #[inline]
    pub fn remove(&mut self, index: Index) -> &mut Self {
        debug_assert!(index < 81);
        let value = index as u128;
        self.state &= !(1u128 << value);
        self
    }

    #[inline]
    pub const fn with_union(mut self, other: &IndexBitSet) -> Self {
        self.state |= other.state;
        self
    }

    #[inline]
    pub fn union(&mut self, other: &IndexBitSet) -> &mut Self {
        self.state |= other.state;
        self
    }

    #[inline]
    pub const fn contains(&self, index: Index) -> bool {
        if index >= 81 {
            return false;
        }

        let value = index as u128;
        let flag = self.state & (1 << value);
        flag != 0
    }

    pub fn len(&self) -> usize {
        self.state.count_ones() as _
    }

    pub fn is_empty(&self) -> bool {
        self.state == 0
    }

    pub fn iter(&self) -> IndexBitSetIter {
        IndexBitSetIter {
            value: self,
            index: 0,
        }
    }
}

impl From<&[u8]> for IndexBitSet {
    #[inline]
    fn from(values: &[u8]) -> Self {
        let mut state = 0u128;
        for value in values {
            state |= 1 << value;
        }
        Self { state }
    }
}

pub struct IndexBitSetIter<'a> {
    value: &'a IndexBitSet,
    index: u8,
}

impl<'a> Iterator for IndexBitSetIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 81 {
            return None;
        }

        while self.index < 81 && !self.value.contains(self.index) {
            self.index += 1;
        }

        let matched = self.index;
        self.index += 1;

        if matched >= 81 {
            None
        } else {
            Some(matched)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::indexbitset::IndexBitSet;

    #[test]
    fn with_value() {
        let a = 80;
        let b = 17;
        let c = 2;

        let bitset = IndexBitSet::default().with_value(a).with_value(b);

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));

        assert_eq!(bitset.len(), 2);
        assert!(!bitset.is_empty());
    }

    #[test]
    fn union() {
        let a = 80;
        let b = 17;
        let c = 2;

        let bitset_a = IndexBitSet::default().with_value(a);
        let bitset_b = IndexBitSet::default().with_value(b);
        let bitset = bitset_a.with_union(&bitset_b);

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));
    }

    #[test]
    fn without_value() {
        let a = 80;
        let b = 17;
        let c = 2;

        let bitset = IndexBitSet::default()
            .with_value(a)
            .with_value(b)
            .with_value(c);
        let bitset = bitset.without_value(a).without_value(b);

        assert!(!bitset.contains(a));
        assert!(!bitset.contains(b));
        assert!(bitset.contains(c));
    }

    #[test]
    fn from_u8_slice() {
        let a = 80;
        let b = 17;
        let c = 2;

        let bitset = IndexBitSet::from([a, b].as_slice());

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));
    }

    #[test]
    fn iter() {
        let a = 80;
        let b = 17;

        let bitset = IndexBitSet::default().with_value(a).with_value(b);
        let mut iter = bitset.iter();

        assert_eq!(iter.next(), Some(17));
        assert_eq!(iter.next(), Some(80));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
