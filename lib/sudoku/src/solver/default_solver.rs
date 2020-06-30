use std::collections::{HashSet, BTreeMap, HashMap};

use crate::prelude::*;
use crate::GameState;
use crate::game::Placement;
use crate::solver::candidates::{find_move_candidates, MoveCandidates};
use crate::solver::steps::{lone_singles, naked_twins, hidden_singles};

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.clone());

    'stack: while let Some(mut state) = stack.pop() {

        let mut candidates = find_move_candidates(&state, &valid_symbols);
        if candidates.is_empty() {
            return state;
        }

        let mut state_changed = false;

        let applied = lone_singles(&mut state, &candidates);
        state_changed |= !applied.is_empty();
        candidates.eliminate_many(applied.into_iter());

        let applied = hidden_singles(&mut state, &candidates);
        state_changed |= !applied.is_empty();
        candidates.eliminate_many(applied.into_iter());

        state_changed |= naked_twins(&mut state, &mut candidates);

        // TODO: If the state didn't change, we need to fork.
        assert!(state_changed);

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