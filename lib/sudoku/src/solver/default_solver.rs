use std::collections::{BTreeSet, HashSet, BTreeMap, HashMap};

use crate::prelude::*;
use crate::GameState;
use crate::game::Move;
use crate::solver::typed_move::TypedMove;
use std::iter::FromIterator;

fn expand_moves(state: &GameState, valid_symbols: &HashSet<Value>) -> Vec<TypedMove> {
    let list_of_lists = find_move_candidates(&state, &valid_symbols);
    flatten_list(list_of_lists)
}

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut tried_moves = HashMap::new();

    let mut best_solution = game.clone();
    let mut best_num_empty = game.empty_cells.len();

    let mut stack = Vec::new();
    stack.push(game.clone());

    tried_moves.insert(game.state.clone(), HashSet::new());

    'stack: while let Some(state) = stack.pop() {
        let num_empty = state.empty_cells.len();
        if num_empty == 0 {
            best_solution = state;
            best_num_empty = 0;
            break 'stack;
        }

        let mut candidates = expand_moves(&state, &valid_symbols);
        println!("{} empty cells to try; stack depth {}; candidates {}", num_empty, stack.len(), candidates.len());

        'moves: while let Some(typed_move) = candidates.pop() {
            let r#move = typed_move.r#move;

            assert!(tried_moves.contains_key(&state.state));
            let lookup = tried_moves.get_mut(&state.state).unwrap();
            if lookup.contains(&r#move) {
                continue 'moves;
            }
            lookup.insert(r#move.clone());

            let next_state = state.place_and_fork(r#move.index, r#move.value);
            if tried_moves.contains_key(&next_state.state) {
                continue 'moves;
            }
            tried_moves.insert(next_state.state.clone(), HashSet::new());

            // If the move is trivial, we can just replace the top of the stack;
            // the removal part of that is already done. However, if we need to branch,
            // we need to re-add our current state to make sure we can revisit it later.
            if typed_move.is_branching && candidates.len() > 0 {
                stack.push(state);
            }
            else {
                println!("Apply trivial move")
            }

            stack.push(next_state);
            continue 'stack;
        }

        // No possible moves were found; this branch is a dead end.
        // Since we already popped the last state from the stack,
        // the next iteration will continue at the previous branch.

        if num_empty < best_num_empty {
            best_num_empty = num_empty;
            best_solution = state.clone();
            println!("New optimum");
        }

        if num_empty == 0 || stack.len() == 0 {
            break 'stack;
        }
    }

    best_solution
}

/// Finds the open cells and returns them in order of descending move options.
fn find_move_candidates(state: &GameState, valid_symbols: &HashSet<u32>) -> Vec<Vec<Move>> {
    let mut open_cells = BTreeMap::new();

    for index in &state.empty_cells {
        let missing_values = collect_missing_values(*index, state, valid_symbols);
        for value in missing_values {
            let r#move = Move::new(value, index.clone());

            open_cells.entry(index.clone()).or_insert_with(|| Vec::new()).push(r#move);
        }
    }

    to_sorted_list(open_cells)
}

fn to_sorted_list(set: BTreeMap<Index, Vec<Move>>) -> Vec<Vec<Move>> {
    let mut out = Vec::new();
    for (_, list) in set {
        out.push(list);
    }

    out.sort_unstable_by_key(|list| -(list.len() as isize));
    //out.sort_unstable_by_key(|list| list.len());
    out
}

fn flatten_list(list_of_list: Vec<Vec<Move>>) -> Vec<TypedMove> {
    let mut out = Vec::new();
    for mut list in list_of_list {
        let branch = list.len() > 1;
        while let Some(r#move) = list.pop() {
            out.push(TypedMove::new(r#move, branch));
        }
    }
    out
}

fn collect_missing_values(index: Index, state: &GameState, valid_symbols: &HashSet<u32>) -> HashSet<Value> {
    let cell_values = state.peers_by_index(index);
    let value_set = to_value_set(cell_values);
    valid_symbols.difference(&value_set).map(move |x| *x).collect()
}

fn collect_valid_symbols(game: &GameState) -> HashSet<Value> {
    let mut symbols = HashSet::new();
    for symbol in game.symbols() {
        symbols.insert(*symbol);
    }
    symbols
}

fn to_value_set(set: HashSet<Move>) -> HashSet<Value> {
    let mut vec = HashSet::new();
    for value in set.into_iter() {
        vec.insert(value.value);
    }
    vec
}

fn to_value_vec(set: HashSet<Move>) -> Vec<Value> {
    let mut vec = Vec::new();
    for value in set.into_iter() {
        vec.push(value.value);
    }
    vec
}