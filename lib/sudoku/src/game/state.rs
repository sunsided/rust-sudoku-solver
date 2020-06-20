use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use visitor::{AcceptVisitor, Visitor};
use crate::Game;
use crate::game::IndexSet;

pub type CellValue = Option<u32>;

pub struct State {
    state: Box<[CellValue; 81]>
}

impl State {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(state: [CellValue; 81]) -> State {
        State { state: Box::new(state) }
    }

    pub fn cell(&self, x: usize, y: usize, width: usize, height: usize) -> CellValue {
        assert!(x < width && y < height);
        self.state[index(x, y, width)]
    }

    pub fn cell_at(&self, index: usize, width: usize, height: usize) -> CellValue {
        assert!(index < width * height);
        self.state[index]
    }

    pub fn fork(&self) -> State {
        State { state: Box::new((*self.state).clone()) }
    }

    pub fn place_and_fork(&self, index: usize, value: u32) -> State {
        let mut state = (*self.state).clone();
        state[index] = Some(value);

        State { state: Box::new(state) }
    }

    pub fn missing(&self) -> IndexSet {
        let mut set = IndexSet::new();
        for index in 0..self.state.len() {
            let cell = &self.state[index];
            if cell.is_none() {
                set.insert(index);
            }
        }
        set
    }
}

pub fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
