use std::rc::Rc;
use visitor::prelude::*;
use crate::game::prelude::*;
use crate::{Game, State};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use crate::game::Move;

pub struct GameState {
    pub empty_cells: IndexSet,
    pub game: Rc<Game>,
    pub state: State
}

impl GameState {
    pub fn new(game: Game) -> GameState {
        let state = game.fork_state();
        let missing = state.empty_cells();
        GameState { game: Rc::new(game), state, empty_cells: missing }
    }

    pub fn peers_by_index(&self, index: Index) -> HashSet<Move> {
        let (x, y) = self.index_to_xy(index);
        self.peers_by_xy(x, y)
    }

    pub fn peers_by_xy(&self, x: Coordinate, y: Coordinate) -> HashSet<Move> {
        let column = self.get_column_values(x);
        let row = self.get_row_values(y);
        let group = self.get_group_values(x, y);
        join_hashset!(column, row, group)
    }

    pub fn place_and_fork(&self, index: usize, value: u32) -> GameState {
        let state = self.state.apply_and_fork(index, value);

        let mut missing = self.state.empty_cells();
        missing.remove(&index);

        GameState { game: self.game.clone(), state, empty_cells: missing }
    }

    pub fn symbols(&self) -> &[u32; 9] {
        self.game.symbols()
    }

    pub fn cell(&self, x: usize, y: usize) -> ValueOption {
        self.state.cell_at_xy(x, y, self.game.width, self.game.height)
    }

    fn get_row_values(&self, y: usize) -> Vec<Move> {
        let mut set = Vec::new();
        for x in 0..self.game.width {
            let index = self.xy_to_index(x, y);
            self.collect_if_set(&mut set, index);
        }
        set
    }

    fn get_column_values(&self, x: usize) -> Vec<Move> {
        let mut set = Vec::new();
        for y in 0..self.game.height {
            let index = self.xy_to_index(x, y);
            self.collect_if_set(&mut set, index);
        }
        set
    }

    fn get_group_values(&self, x: usize, y: usize) -> Vec<Move> {
        let width = self.game.width;
        let height = self.game.height;
        let mut set = Vec::new();
        let group = &self.game.group_at(x, y);
        for index in group.iter() {
            self.collect_if_set(&mut set, *index);
        }
        set
    }

    fn collect_if_set(&self, set: &mut Vec<Move>, index: Index) {
        if let Some(value) = self.state.cell_at_index(index, self.game.width, self.game.height) {
            set.push(Move::new(value, index));
        }
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.game.width
    }

    pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
        let x = index % self.game.width;
        let y = index / self.game.width;
        (x, y)
    }

    pub fn validate(&self) -> bool {
        let mut valid = true;
        for y in 0..self.game.height {
            valid &= self.validate_row(y);
        }

        for x in 0..self.game.width {
            valid &= self.validate_column(x);
        }

        for group in self.game.groups.as_slice() {
            valid &= self.validate_group(group);
        }

        valid
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
        values.len() == self.game.width
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

impl Clone for GameState {
    fn clone(&self) -> Self {
        let missing = self.state.empty_cells();
        GameState {
            game: self.game.clone(),
            state: self.state.clone(),
            empty_cells: missing }
    }
}

impl AcceptVisitor<GameState> for GameState {
    fn accept<V: Visitor<GameState>>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state)
    }
}

impl Eq for GameState {}
