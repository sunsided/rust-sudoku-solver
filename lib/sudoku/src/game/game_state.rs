use crate::game::indexbitset::IndexBitSet;
use crate::game::prelude::*;
use crate::game::state::StateId;
use crate::game::{Placement, ValueBitSet};
use crate::{Game, State};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use visitor::prelude::*;

pub enum CollectType {
    All,
    Empty,
    Filled,
}

pub struct GameState {
    pub empty_cells: IndexBitSet,
    pub game: Rc<Game>,
    pub state: State,
}

impl GameState {
    pub fn new(game: Game) -> GameState {
        let state = game.fork_state();
        let missing = state.empty_cells();
        GameState {
            game: Rc::new(game),
            state,
            empty_cells: missing,
        }
    }

    pub fn peers_by_index(&self, index: Index, exclude_self: bool) -> HashSet<Placement> {
        let (x, y) = self.index_to_xy(index);
        self.peers_by_xy(x, y, exclude_self)
    }

    pub fn peer_indexes_by_index(
        &self,
        index: Index,
        exclude_self: bool,
        how: CollectType,
    ) -> HashSet<Index> {
        let (x, y) = self.index_to_xy(index);
        self.peer_indexes_by_xy(x, y, exclude_self, how)
    }

    pub fn peers_by_xy(
        &self,
        x: Coordinate,
        y: Coordinate,
        exclude_self: bool,
    ) -> HashSet<Placement> {
        let column = self.get_column_values(x, y, exclude_self);
        let row = self.get_row_values(x, y, exclude_self);
        let group = self.get_group_values(x, y, exclude_self);
        join_hashset!(column, row, group)
    }

    pub fn peer_indexes_by_xy(
        &self,
        x: Coordinate,
        y: Coordinate,
        exclude_self: bool,
        how: CollectType,
    ) -> HashSet<Index> {
        let column = self.get_column_indexes(x, y, exclude_self, &how);
        let row = self.get_row_indexes(x, y, exclude_self, &how);
        let group = self.get_group_indexes(x, y, exclude_self, &how);
        join_hashset!(column, row, group)
    }

    pub fn apply(&mut self, index: u8, value: Value) {
        self.state.apply(index, value);
        self.empty_cells.remove(index);
    }

    pub fn apply_move(&mut self, r#move: &Placement) {
        self.apply(r#move.index, r#move.value)
    }

    pub fn apply_and_fork(&self, index: Index, value: Value) -> GameState {
        let state = self.state.apply_and_fork(index, value);

        let missing = state.empty_cells();
        debug_assert!(!missing.contains(index));

        GameState {
            game: self.game.clone(),
            state,
            empty_cells: missing,
        }
    }

    pub fn valid_symbols(&self) -> &[Value; 9] {
        self.game.valid_symbols()
    }

    pub fn cell(&self, x: usize, y: usize) -> ValueOption {
        self.state
            .cell_at_xy(x, y, self.game.width, self.game.height)
    }

    pub fn id(&self) -> &StateId {
        &self.state.id
    }

    fn get_row_values(
        &self,
        x_reference: Coordinate,
        y: Coordinate,
        exclude_self: bool,
    ) -> Vec<Placement> {
        let mut set = Vec::new();
        for x in 0..self.game.width {
            if exclude_self && (x == x_reference) {
                continue;
            }

            let index = self.xy_to_index(x, y);
            self.collect_if_set(&mut set, index);
        }
        set
    }

    fn get_row_indexes(
        &self,
        x_reference: Coordinate,
        y: Coordinate,
        exclude_self: bool,
        how: &CollectType,
    ) -> Vec<Index> {
        let mut set = Vec::new();
        for x in 0..self.game.width {
            if exclude_self && (x == x_reference) {
                continue;
            }

            let index = self.xy_to_index(x, y);
            self.collect_index_if(&mut set, index, &how);
        }
        set
    }

    fn get_column_values(
        &self,
        x: Coordinate,
        y_reference: Coordinate,
        exclude_self: bool,
    ) -> Vec<Placement> {
        let mut set = Vec::new();
        for y in 0..self.game.height {
            if exclude_self && (y == y_reference) {
                continue;
            }

            let index = self.xy_to_index(x, y);
            self.collect_if_set(&mut set, index);
        }
        set
    }

    fn get_column_indexes(
        &self,
        x: Coordinate,
        y_reference: Coordinate,
        exclude_self: bool,
        how: &CollectType,
    ) -> Vec<Index> {
        let mut set = Vec::new();
        for y in 0..self.game.height {
            if exclude_self && (y == y_reference) {
                continue;
            }

            let index = self.xy_to_index(x, y);
            self.collect_index_if(&mut set, index, &how);
        }
        set
    }

    fn get_group_values(&self, x: Coordinate, y: Coordinate, exclude_self: bool) -> Vec<Placement> {
        let mut set = Vec::new();
        let group = &self.game.group_at(x, y);
        let index_reference = self.xy_to_index(x, y);

        for index in group.iter() {
            if exclude_self && (index == index_reference) {
                continue;
            }

            self.collect_if_set(&mut set, index);
        }
        set
    }

    fn get_group_indexes(
        &self,
        x: Coordinate,
        y: Coordinate,
        exclude_self: bool,
        how: &CollectType,
    ) -> Vec<Index> {
        let mut set = Vec::new();
        let group = &self.game.group_at(x, y);
        let index_reference = self.xy_to_index(x, y);

        for index in group.iter() {
            if exclude_self && (index == index_reference) {
                continue;
            }

            self.collect_index_if(&mut set, index, &how);
        }
        set
    }

    fn cell_at_index(&self, index: Index) -> ValueOption {
        self.state
            .cell_at_index(index, self.game.width, self.game.height)
    }

    fn collect_if_set(&self, set: &mut Vec<Placement>, index: Index) {
        if let Some(value) = self.cell_at_index(index) {
            set.push(Placement::new(value, index));
        }
    }

    fn collect_index_if(&self, set: &mut Vec<Index>, index: Index, condition: &CollectType) {
        let cell = self.cell_at_index(index);
        match condition {
            CollectType::All => set.push(index),
            CollectType::Empty => {
                if cell.is_none() {
                    set.push(index);
                }
            }
            CollectType::Filled => {
                if cell.is_some() {
                    set.push(index);
                }
            }
        }
    }

    pub fn xy_to_index(&self, x: usize, y: usize) -> Index {
        (x + y * self.game.width) as Index
    }

    #[inline]
    pub fn index_to_xy(&self, index: Index) -> (usize, usize) {
        let x = (index as usize) % self.game.width;
        let y = (index as usize) / self.game.width;
        (x, y)
    }

    pub fn validate(&self, allow_empty: bool) -> bool {
        let mut valid = true;
        for y in 0..self.game.height {
            valid &= self.validate_row(y, allow_empty);
        }

        for x in 0..self.game.width {
            valid &= self.validate_column(x, allow_empty);
        }

        for group in self.game.groups.as_slice() {
            valid &= self.validate_group(group, allow_empty);
        }

        valid
    }

    fn validate_row(&self, y: Coordinate, allow_empty: bool) -> bool {
        let mut values = ValueBitSet::default();
        for item in self.get_row_values(0, y, false) {
            if values.contains(item.value) {
                return false;
            }
            values.insert(item.value);
        }
        values.len() == self.game.width || allow_empty
    }

    fn validate_column(&self, x: Coordinate, allow_empty: bool) -> bool {
        let mut values = ValueBitSet::default();
        for item in self.get_column_values(x, 0, false) {
            if values.contains(item.value) {
                return false;
            }
            values.insert(item.value);
        }
        values.len() == self.game.height || allow_empty
    }

    fn validate_group(&self, group: &IndexBitSet, allow_empty: bool) -> bool {
        let mut values = ValueBitSet::default();
        let (x, y) = self.index_to_xy(group.iter().next().unwrap());
        for item in self.get_group_values(x, y, false) {
            if values.contains(item.value) {
                return false;
            }
            values.insert(item.value);
        }
        values.len() == group.len() || allow_empty
    }
}

impl Clone for GameState {
    fn clone(&self) -> Self {
        let missing = self.state.empty_cells();
        GameState {
            game: self.game.clone(),
            state: self.state.clone(),
            empty_cells: missing,
        }
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
