use std::collections::{HashSet, BTreeMap, HashMap};

use crate::prelude::*;
use crate::GameState;
use crate::game::Placement;
use crate::solver::move_candidates::MoveCandidates;
use crate::solver::find_move_candidates::find_move_candidates;
use crate::solver::simple_moves::simple_moves;

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.clone());

    'stack: while let Some(mut state) = stack.pop() {

        let mut candidates = find_move_candidates(&state, &valid_symbols);
        if candidates.is_empty() {
            return state;
        }

        simple_moves(&mut state, &mut candidates);

        stack.push(state);
    }

    panic!()
}

fn collect_valid_symbols(game: &GameState) -> HashSet<Value> {
    let mut symbols = HashSet::new();
    for symbol in game.valid_symbols() {
        symbols.insert(*symbol);
    }
    symbols
}

fn to_value_vec(set: HashSet<Placement>) -> Vec<Value> {
    let mut vec = Vec::new();
    for value in set.into_iter() {
        vec.push(value.value);
    }
    vec
}