use std::collections::{BTreeSet, HashSet, BTreeMap};
use std::collections::hash_map::RandomState;

use crate::prelude::*;
use crate::GameState;
use crate::game::Move;
use crate::solver::typed_move::TypedMove;

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.fork());

    let mut tried_moves = HashSet::new();

    'stack: while let Some(state) = stack.pop() {
        let list_of_lists = find_move_candidates(&state, &valid_symbols);
        let candidates = flatten_list(list_of_lists);
        println!("{} more moves to try", candidates.len());

        if candidates.is_empty() {
            stack.push(state);
            break 'stack;
        }

        'moves: for typed_move in candidates {
            let r#move = typed_move.r#move;
            if tried_moves.contains(&r#move) {
                continue 'moves;
            }

            let next_state = state.place_and_fork(r#move.index, r#move.value);
            if !typed_move.trivial {
                stack.push(state);
            }

            stack.push(next_state);
            tried_moves.insert(r#move);
            continue 'stack;
        }

        // No possible moves were found; this branch is a dead end.
        // Since we already popped the last state from the stack,
        // the next iteration will continue at the previous branch.
    }

    let final_state = stack.pop();
    final_state.unwrap()
}

fn apply_trivial_moves(state: GameState, trivial_cells: Vec<(usize, u32)>) -> GameState {
    trivial_cells.iter().fold(state, |current, (index, candidates)|
        current.place_and_fork(*index, *candidates))
}

/// Finds the open cells and returns them in order of descending move options.
fn find_move_candidates(state: &GameState, valid_symbols: &BTreeSet<u32>) -> Vec<Vec<Move>> {
    let mut open_cells = BTreeMap::new();

    for index in &state.empty_cells {
        let missing_values = collect_missing_values(index, state, valid_symbols);

        let (x, y) = state.index_to_xy(index.clone());
        for value in missing_values {
            let r#move = Move::new(value, index.clone(), x.clone(), y.clone());

            open_cells.entry(index.clone()).or_insert_with(|| Vec::new()).push(r#move);
        }
    }

    // Order by possible moves, asscending.
    to_sorted_list(open_cells)
}

fn to_sorted_list(set: BTreeMap<Index, Vec<Move>>) -> Vec<Vec<Move>> {
    let mut out = Vec::new();
    for (_, list) in set {
        out.push(list);
    }

    out.sort_unstable_by_key(|list| list.len());
    out
}

fn flatten_list(list_of_list: Vec<Vec<Move>>) -> Vec<TypedMove> {
    let mut out = Vec::new();
    for mut list in list_of_list {
        let trivial = list.len() == 1;
        while let Some(r#move) = list.pop() {
            out.push(TypedMove::new(r#move, trivial));
        }
    }
    out
}

fn collect_missing_values(index: &usize, state: &GameState, valid_symbols: &BTreeSet<u32>) -> Vec<Value> {
    let (x, y) = state.index_to_xy(*index);

    // Determine the symbols used in the context of the current cell.
    let column = state.get_column_values(x);
    let row = state.get_row_values(y);
    let group = state.get_group_values(x, y);

    // Determine the remaining possible moves for the current cell.
    let values = join_btreeset!(column, row, group);
    valid_symbols.difference(&values).map(|x| *x).collect()
}

fn collect_valid_symbols(game: &GameState) -> BTreeSet<Value> {
    let mut symbols = BTreeSet::new();
    for symbol in game.symbols() {
        symbols.insert(*symbol);
    }
    symbols
}

