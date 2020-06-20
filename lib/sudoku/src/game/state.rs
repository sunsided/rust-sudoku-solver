use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use visitor::{AcceptVisitor, Visitor};
use crate::Game;

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

    pub fn fork(&self) -> State {
        State { state: Box::new((*self.state).clone()) }
    }
}

pub fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
