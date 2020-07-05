use crate::prelude::*;
use crate::{GameState, Placement};
use crate::solver::candidates::SetOfMoveCandidates;
use crate::game::CollectType;
use std::collections::HashSet;

pub fn naked_twins(state: &mut GameState, candidates: &mut SetOfMoveCandidates) -> bool {
    for candidate in candidates.iter() {
        let peer_indexes = state.peer_indexes_by_index(candidate.index, true, CollectType::Empty);
        for index in peer_indexes {
            // If no suggestion exists for the peer field, the current branch is unsolvable.
            // We assume that this is known beforehand.
            if !candidates.contains_key(&index) {
                continue;
            }

            let peers = &candidates[index];

            // A naked twin condition only occurs if any of the peers has the
            // same number of elements.
            if peers.len() == candidate.len() {

                // In addition, all the values need to be identical.
                let num_different = candidate.moves.difference(&peers).count();
                if num_different == 0 {
                    assert!(false);
                }

                // TODO: Now do something with this knowledge. :)
            }
        }
    }

    false
}