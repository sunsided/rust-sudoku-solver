use std::rc::Rc;
use visitor::prelude::*;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use crate::{Game, State, CellValue};

pub struct GameState {
    game: Rc<Game>,
    state: State
}

impl GameState {
    pub fn new(game: Game) -> GameState {
        let state = game.fork_state();
        GameState { game: Rc::new(game), state }
    }

    pub fn fork(&self) -> GameState {
        GameState { game: self.game.clone(), state: self.state.fork() }
    }

    pub fn cell(&self, x: usize, y: usize) -> CellValue {
        self.state.cell(x, y, self.game.width, self.game.height)
    }

    pub fn get_row_values(&self, y: usize, state: &[CellValue; 81]) -> HashSet<CellValue, RandomState> {
        let width = self.game.width;
        let mut set = HashSet::new();
        for x in 0..width {
            set.insert(state[index(x, y, width)]);
        }
        set
    }

    pub fn get_column_values(&self, x: usize, state: &[CellValue; 81]) -> HashSet<CellValue, RandomState> {
        let width = self.game.width;
        let height = self.game.height;
        let mut set = HashSet::new();
        for y in 0..height {
            set.insert(state[index(x, y, width)]);
        }
        set
    }

    pub fn get_group_values(&self, x: usize, y: usize, state: &[CellValue; 81]) -> HashSet<CellValue, RandomState> {
        let mut set = HashSet::new();
        let group = &self.game.group_at(x, y);
        for index in group.iter() {
            set.insert(state[*index]);
        }
        set
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
