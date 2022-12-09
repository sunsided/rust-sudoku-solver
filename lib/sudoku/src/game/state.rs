use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct State {
    pub id: StateId,
    values: [ValueOption; 81],
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct StateId {
    repr: [u32; 9],
}

impl State {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(values: [ValueOption; 81]) -> State {
        let id = Self::make_id(&values);
        State { values, id }
    }

    pub fn cell_at_xy(&self, x: usize, y: usize, width: usize, height: usize) -> ValueOption {
        debug_assert!(x < width && y < height);
        self.values[index(x, y, width)]
    }

    pub fn cell_at_index(&self, index: usize, width: usize, height: usize) -> ValueOption {
        debug_assert!(index < width * height);
        self.values[index]
    }

    pub fn apply(&mut self, index: usize, value: Value) {
        self.values[index] = Some(value);
    }

    pub fn apply_and_fork(&self, index: usize, value: Value) -> State {
        let mut state = self.values.clone();
        state[index] = Some(value);
        let id = Self::make_id(&self.values);
        State { values: state, id }
    }

    pub fn empty_cells(&self) -> IndexSet {
        let mut set = IndexSet::new(); // TODO: Use bitset
        for index in 0..self.values.len() {
            if self.values[index].is_none() {
                set.insert(index);
            }
        }
        set
    }

    fn make_id(values: &[ValueOption]) -> StateId {
        // u32 is enough because 9 * 10^8 =   900_000_000
        //   and u32::MAX is              = 4_294_967_295;
        let mut id = [0u32; 9];
        let mut row_index = 0;
        let mut power = 0;
        for value in values.iter() {
            if let Some(value) = value {
                id[row_index] += value.get() as u32 * 10u32.pow(power);
            }

            power += 1;

            // Reset the counter when we reach the end of the row.
            if power == 9 {
                row_index += 1;
                power = 0;
            }
        }

        StateId { repr: id }
    }
}

impl Hash for StateId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for v in self.repr {
            v.hash(state)
        }
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            values: self.values.clone(),
            id: self.id.clone(),
        }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        // self.values.eq(&other.values)
        self.id.eq(&other.id)
    }
}

impl Eq for State {}
