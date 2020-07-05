use crate::game::prelude::*;
use std::hash::{Hash, Hasher};

pub struct State {
    pub id: String,
    values: [ValueOption; 81]
}

impl State {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(values: [ValueOption; 81]) -> State
    {
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

    pub fn apply(&mut self, index: usize, value: u32) {
        self.values[index] = Some(value);
    }

    pub fn apply_and_fork(&self, index: usize, value: u32) -> State {
        let mut state = self.values.clone();
        state[index] = Some(value);
        let id = Self::make_id(&self.values);
        State { values: state, id }
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

    fn make_id(values: &[ValueOption]) -> String {
        let mut str = String::new();
        for value in values.iter() {
            let value_unwrapped = if value.is_some() { value.unwrap() + 1 } else { 0 };
            str += format!("{}.", value_unwrapped).as_str();
        }
        str
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State { values: self.values.clone(), id: self.id.clone() }
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