use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct State {
    values: [ValueOption; 81]
}

impl State {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(values: [ValueOption; 81]) -> State {
        State { values }
    }

    pub fn cell_at_xy(&self, x: usize, y: usize, width: usize, height: usize) -> ValueOption {
        assert!(x < width && y < height);
        self.values[index(x, y, width)]
    }

    pub fn cell_at_index(&self, index: usize, width: usize, height: usize) -> ValueOption {
        assert!(index < width * height);
        self.values[index]
    }

    pub fn apply(&mut self, index: usize, value: u32) {
        self.values[index] = Some(value);
    }

    pub fn apply_and_fork(&self, index: usize, value: u32) -> State {
        let mut state = self.values.clone();
        state[index] = Some(value);
        State { values: state }
    }

    pub fn empty_cells(&self) -> IndexSet {
        let mut set = IndexSet::new();
        for index in 0..self.values.len() {
            if self.values[index].is_none() {
                set.insert(index);
            }
        }
        set
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State { values: self.values.clone() }
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