use std::rc::Rc;
use visitor::prelude::*;
use crate::game::prelude::*;
use crate::{Game, State};
use std::collections::HashSet;

pub struct GameState {
    pub empty_cells: IndexSet,
    game: Rc<Game>,
    state: State
}

impl GameState {
    pub fn new(game: Game) -> GameState {
        let state = game.fork_state();
        let missing = state.missing();
        GameState { game: Rc::new(game), state, empty_cells: missing }
    }

    pub fn fork(&self) -> GameState {
        let state = self.state.fork();
        let missing = self.state.missing();
        GameState { game: self.game.clone(), state, empty_cells: missing }
    }

    pub fn place_and_fork(&self, index: usize, value: u32) -> GameState {
        let state = self.state.place_and_fork(index, value);

        let mut missing = self.state.missing();
        missing.remove(&index);

        GameState { game: self.game.clone(), state, empty_cells: missing }
    }

    pub fn symbols(&self) -> &[u32; 9] {
        self.game.symbols()
    }

    pub fn cell(&self, x: usize, y: usize) -> CellValue {
        self.state.cell(x, y, self.game.width, self.game.height)
    }

    pub fn get_row_values(&self, y: usize) -> Vec<u32> {
        let width = self.game.width;
        let height = self.game.height;
        let mut set = Vec::new();
        for x in 0..width {
            if let Some(value) = self.state.cell(x, y, width, height) {
                set.push(value);
            }
        }
        set
    }

    pub fn get_column_values(&self, x: usize) -> Vec<Value> {
        let width = self.game.width;
        let height = self.game.height;
        let mut set = Vec::new();
        for y in 0..height {
            if let Some(value) = self.state.cell(x, y, width, height) {
                set.push(value);
            }
        }
        set
    }

    pub fn get_group_values(&self, x: usize, y: usize) -> Vec<u32> {
        let width = self.game.width;
        let height = self.game.height;
        let mut set = Vec::new();
        let group = &self.game.group_at(x, y);
        for index in group.iter() {
            if let Some(value) = self.state.cell_at(*index, width, height) {
                set.push(value);
            }
        }
        set
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.game.width
    }

    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.game.width;
        let y = index / self.game.width;
        (x, y)
    }

    pub fn validate(&self) -> bool{
        for y in 0..self.game.height {
            if !self.validate_row(y) {
                return false
            }
        }

        for x in 0..self.game.width {
            if !self.validate_column(x) {
                return false
            }
        }

        for group in self.game.groups.as_slice() {
            if !self.validate_group(group) {
                return false
            }
        }

        true
    }

    fn validate_row(&self, y: Coordinate) -> bool{
        let mut values = HashSet::new();
        for item in self.get_row_values(y) {
            values.insert(item);
        }
        values.len() == self.game.height
    }

    fn validate_column(&self, x: Coordinate) -> bool{
        let mut values = HashSet::new();
        for item in self.get_column_values(x) {
            values.insert(item);
        }
        values.len() == self.game.height
    }

    fn validate_group(&self, group: &IndexSet) -> bool{
        let mut values = HashSet::new();
        let (x, y) = self.index_to_xy(*group.iter().next().unwrap());
        for item in self.get_group_values(x, y) {
            values.insert(item);
        }
        values.len() == group.len()
    }
}

impl AcceptVisitor<GameState> for GameState {
    fn accept<V: Visitor<GameState>>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}
