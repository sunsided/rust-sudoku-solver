use crate::prelude::*;
use crate::GameState;
use std::collections::HashSet;
use crate::solver::move_candidates::MoveCandidates;
use crate::solver::set_of_move_candidates::SetOfMoveCandidates;

pub fn simple_moves(state: &mut GameState, candidates: &mut SetOfMoveCandidates) {
    let mut drained = Vec::new();

    for candidate in candidates.iter() {
        if !candidate.is_trivial() {
            continue;
        }

        for r#move in candidate.moves {
            state.apply_move(&r#move);
        }

        drained.push(candidate.index);
    }

    candidates.remove_candidates(drained.into_iter());
}
