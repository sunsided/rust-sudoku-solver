use crate::prelude::{Value, ValueOption};

/// A simple bitset for storing regular Sudoku-sized (i.e., up to 9) cell values.
///
/// ## Technical Notes
/// Practically this implementation allows for storing up to 65535 different indexes.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ValueBitSet {
    /// We anticipate at most 9 distinct values on a standard Sudoku game.
    /// We use a 16-bit type here to directly encode the field values,
    /// even though this wastes 7 bits.
    state: u16,
}

impl ValueBitSet {
    #[inline]
    pub const fn with_value(mut self, value: Value) -> Self {
        debug_assert!(value.get() <= 9);
        let value = value.get() as u16;
        // Since the value is a non-zero u8 we subtract one for the first bit.
        self.state |= 1u16 << (value - 1);
        self
    }

    #[inline]
    pub fn insert(&mut self, value: Value) -> &mut Self {
        debug_assert!(value.get() <= 9);
        let value = value.get() as u16;
        // Since the value is a non-zero u8 we subtract one for the first bit.
        self.state |= 1u16 << (value - 1);
        self
    }

    #[inline]
    pub const fn without_value(mut self, value: Value) -> Self {
        debug_assert!(value.get() <= 9);
        let value = value.get() as u128;
        // Since the value is a non-zero u8 we subtract one for the first bit.
        self.state &= !(1u16 << (value - 1));
        self
    }

    #[inline]
    pub fn remove(&mut self, value: Value) -> &mut Self {
        debug_assert!(value.get() <= 9);
        let value = value.get() as u16;
        // Since the value is a non-zero u8 we subtract one for the first bit.
        self.state &= !(1u16 << (value - 1));
        self
    }

    #[inline]
    pub const fn with_union(mut self, other: &ValueBitSet) -> Self {
        self.state |= other.state;
        self
    }

    #[inline]
    pub fn union(&mut self, other: &ValueBitSet) -> &mut Self {
        self.state |= other.state;
        self
    }

    #[inline]
    pub const fn contains(&self, value: Value) -> bool {
        debug_assert!(value.get() <= 9);
        let value = value.get() as u16;
        // Since the value is a non-zero u8 we subtract one for the first bit.
        let flag = self.state & (1u16 << (value - 1));
        flag != 0
    }

    pub fn len(&self) -> usize {
        self.state.count_ones() as _
    }

    pub fn is_empty(&self) -> bool {
        self.state == 0
    }

    pub fn iter(&self) -> ValueBitSetIter {
        ValueBitSetIter {
            value: self,
            index: 1, // Zero is invalid!
        }
    }
}

impl From<&[u8]> for ValueBitSet {
    #[inline]
    fn from(values: &[u8]) -> Self {
        let mut state = 0u16;
        for value in values {
            debug_assert_ne!(*value, 0);
            state |= 1 << (value - 1);
        }
        Self { state }
    }
}

pub struct ValueBitSetIter<'a> {
    value: &'a ValueBitSet,
    index: u8,
}

impl<'a> Iterator for ValueBitSetIter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert_ne!(self.index, 0);
        if self.index > 9 {
            return None;
        }

        while self.index < 9
            && !self
                .value
                .contains(unsafe { Value::new_unchecked(self.index) })
        {
            self.index += 1;
        }

        let matched = self.index;
        self.index += 1;

        if matched > 9 {
            None
        } else {
            Some(Value::try_from(matched).unwrap())
        }
    }
}

impl From<&[Value]> for ValueBitSet {
    #[inline]
    fn from(values: &[Value]) -> Self {
        let mut state = 0u16;
        for value in values {
            // Since the value is a non-zero u8 we subtract one for the first bit.
            state |= 1u16 << (value.get() - 1);
        }
        Self { state }
    }
}

impl From<&[ValueOption]> for ValueBitSet {
    #[inline]
    fn from(values: &[ValueOption]) -> Self {
        let mut state = 0u16;
        for value in values {
            if let Some(value) = value {
                // Since the value is a non-zero u8 we subtract one for the first bit.
                state |= 1u16 << (value.get() - 1);
            }
        }
        Self { state }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::ValueBitSet;
    use crate::prelude::Value;

    #[test]
    fn with_value() {
        let a = Value::try_from(9).unwrap();
        let b = Value::try_from(5).unwrap();
        let c = Value::try_from(2).unwrap();

        let bitset = ValueBitSet::default().with_value(a).with_value(b);

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));

        assert_eq!(bitset.len(), 2);
        assert!(!bitset.is_empty());
    }

    #[test]
    fn union() {
        let a = Value::try_from(9).unwrap();
        let b = Value::try_from(5).unwrap();
        let c = Value::try_from(2).unwrap();

        let bitset_a = ValueBitSet::default().with_value(a);
        let bitset_b = ValueBitSet::default().with_value(b);
        let bitset = bitset_a.with_union(&bitset_b);

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));
    }

    #[test]
    fn without_value() {
        let a = Value::try_from(9).unwrap();
        let b = Value::try_from(5).unwrap();
        let c = Value::try_from(2).unwrap();

        let bitset = ValueBitSet::default()
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
        let a = Value::try_from(9).unwrap();
        let b = Value::try_from(5).unwrap();
        let c = Value::try_from(2).unwrap();

        let bitset = ValueBitSet::from([a, b].as_slice());

        assert!(bitset.contains(a));
        assert!(bitset.contains(b));
        assert!(!bitset.contains(c));
    }

    #[test]
    fn iter() {
        let a = Value::try_from(9).unwrap();
        let b = Value::try_from(5).unwrap();

        let bitset = ValueBitSet::default().with_value(a).with_value(b);
        let mut iter = bitset.iter();

        assert_eq!(iter.next(), Some(b));
        assert_eq!(iter.next(), Some(a));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
