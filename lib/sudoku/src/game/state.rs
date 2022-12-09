use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct State {
    pub id: StateId,
    values: [ValueOption; 81],
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct StateId {
    repr: [usize; 9],
}

impl State {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(values: [ValueOption; 81]) -> State {
        let id = Self::make_id(&values);
        State { values, id }
    }

    pub fn cell_at_xy(&self, x: usize, y: usize, width: usize, height: usize) -> ValueOption {
        assert!(x < width && y < height);
        self.values[index(x, y, width)]
    }

    pub fn cell_at_index(&self, index: usize, width: usize, height: usize) -> ValueOption {
        assert!(index < width * height);
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
        let mut id = [0usize; 9];
        let mut row_index = 0;
        let mut power = 0;
        for value in values.iter() {
            if let Some(value) = value {
                id[row_index] += value.get() as usize * 10usize.pow(power);
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
        let values = &self.values;
        for i in 0..values.len() {
            values[i].hash(state);
        }
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        let values = &self.values;
        if values.len() != other.values.len() {
            return false;
        }

        let mut equality = true;
        for index in 0..values.len() {
            equality &= values[index].eq(&other.values[index])
        }

        equality
    }
}

impl Eq for State {}
