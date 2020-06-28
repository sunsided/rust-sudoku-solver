use std::collections::{HashSet, BTreeMap};
use std::iter::FromIterator;
use crate::prelude::*;
use crate::GameState;
use crate::game::Placement;
use crate::solver::candidates::{MoveCandidates, SetOfMoveCandidates};


/// Finds the open cells and returns them in order of descending move options.
pub fn find_move_candidates(state: &GameState, valid_symbols: &HashSet<Value>) -> SetOfMoveCandidates {
    let mut candidates = SetOfMoveCandidates::new();

    for index in &state.empty_cells {
        let missing_values = collect_missing_values(*index, state, valid_symbols);
        for value in missing_values {
            let r#move = Placement::new(value, index.clone());
            candidates.add_candidate(r#move);
        }
    }

    candidates
}

fn collect_missing_values(index: Index, state: &GameState, valid_symbols: &HashSet<Value>) -> HashSet<Value> {
    let cell_values = state.peers_by_index(index);
    let value_set = to_value_set(cell_values);
    valid_symbols.difference(&value_set).map(move |x| *x).collect()
}

fn to_value_set(set: HashSet<Placement>) -> HashSet<Value> {
    HashSet::from_iter(set.into_iter().map(|v| v.value))
}
