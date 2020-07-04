use std::collections::{HashSet, BTreeMap, HashMap};

use crate::prelude::*;
use crate::GameState;
use crate::game::Placement;
use crate::solver::candidates::{find_move_candidates, MoveCandidates, SetOfMoveCandidates};
use crate::solver::steps::{lone_singles, naked_twins, hidden_singles};
use std::iter::FromIterator;

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.clone());

    let mut tried_moves = HashSet::new();

    'stack: while let Some(mut state) = stack.pop() {

        let mut candidates = find_move_candidates(&state, &valid_symbols);
        if candidates.is_empty() {
            return state;
        }

        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        let mut state_changed = false;

        // Apply lone singles strategy.
        let applied = lone_singles(&mut state, &candidates);
        state_changed |= !applied.is_empty();
        candidates.eliminate_many(applied.into_iter());

        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        // Apply hidden singles strategy.
        let applied = hidden_singles(&mut state, &candidates);
        state_changed |= !applied.is_empty();
        candidates.eliminate_many(applied.into_iter());

        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        // Apply naked twins strategy.
        state_changed |= naked_twins(&mut state, &mut candidates);

        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        if state_changed {
            stack.push(state);
            continue 'stack;
        }

        // If the state didn't change, we need to fork.
        let mut sorted_candidates: Vec<MoveCandidates> = Vec::from_iter(candidates.iter());
        sorted_candidates.sort_by_key(|v| v.moves.len());

        for candidate_set in sorted_candidates {
            for candidate in candidate_set.moves {
                let key = (state.id().clone(), candidate.clone());
                if tried_moves.contains(&key) {
                    continue;
                }

                tried_moves.insert(key);

                let fork = state.apply_and_fork(candidate.index, candidate.value);
                stack.push(state);
                stack.push(fork);
                continue 'stack;
            }
        }

        // All possible options were exhausted - this branch is a dead end.
    }

    panic!()
}

fn is_solvable(state: &GameState, candidates: &SetOfMoveCandidates) -> bool {
    state.empty_cells.len() == candidates.len()
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