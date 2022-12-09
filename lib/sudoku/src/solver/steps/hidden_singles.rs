use crate::solver::candidates::SetOfMoveCandidates;
use crate::{GameState, Placement};
use std::collections::HashSet;

pub fn hidden_singles(state: &mut GameState, candidates: &SetOfMoveCandidates) -> Vec<Placement> {
    let mut applied = Vec::new();

    for candidate in candidates.iter() {
        let value_candidates: HashSet<_> = candidate.moves.iter().map(|p| p.value).collect();
        let peer_values: HashSet<_> = state
            .peers_by_index(candidate.index, true)
            .iter()
            .map(|p| p.value)
            .collect();

        let mut difference: HashSet<_> = value_candidates.difference(&peer_values).collect();

        if difference.len() == 1 {
            let value = *difference.drain().next().unwrap();
            applied.push(Placement::new(value, candidate.index));
        }
    }

    applied
}
