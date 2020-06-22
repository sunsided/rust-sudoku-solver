use std::rc::Rc;
use visitor::prelude::*;
use std::collections::BTreeSet;
use crate::{Game, State, CellValue};
use crate::game::game::IndexSet;

pub struct GameState {
    pub missing: IndexSet,
    game: Rc<Game>,
    state: State
}

impl GameState {
    pub fn new(game: Game) -> GameState {
        let state = game.fork_state();
        let missing = state.missing();
        GameState { game: Rc::new(game), state, missing }
    }

    pub fn fork(&self) -> GameState {
        let state = self.state.fork();
        let missing = self.state.missing();
        GameState { game: self.game.clone(), state, missing }
    }

    pub fn place_and_fork(&self, index: usize, value: u32) -> GameState {
        let state = self.state.place_and_fork(index, value);

        let mut missing = self.state.missing();
        missing.remove(&index);

        GameState { game: self.game.clone(), state, missing }
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

    pub fn get_column_values(&self, x: usize) -> Vec<u32> {
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
}

impl AcceptVisitor<GameState> for GameState {
    fn accept<V: Visitor<GameState>>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}

fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}
