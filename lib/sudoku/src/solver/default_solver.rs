use std::collections::{HashSet, BTreeMap, HashMap};

use crate::prelude::*;
use crate::GameState;
use crate::game::Placement;
use crate::solver::candidates::{find_move_candidates, MoveCandidates};
use crate::solver::steps::simple_moves;

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.clone());

    'stack: while let Some(mut state) = stack.pop() {

        let mut candidates = find_move_candidates(&state, &valid_symbols);
        if candidates.is_empty() {
            return state;
        }

        let applied = simple_moves(&mut state, &candidates);
        assert!(applied.len() > 0);
        candidates.eliminate_many(applied.into_iter());

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