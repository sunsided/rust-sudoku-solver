use crate::prelude::*;
use crate::{GameState, Placement};
use crate::solver::candidates::SetOfMoveCandidates;

pub fn naked_twins(state: &mut GameState, candidates: &mut SetOfMoveCandidates) {
    for candidate in candidates.iter() {
        let num_moves = candidates.len();
        let peers = state.peers_by_index(candidate.index, true)
            .into_iter()
            .map(|placement| placement.index)
            .filter(|index| candidates[*index].len() == num_moves);
    }
}