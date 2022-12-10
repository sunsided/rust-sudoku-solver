use crate::solver::candidates::SetOfMoveCandidates;
use crate::solver::steps::{StrategyError, StrategyMove};
use crate::{GameState, Placement};
use log::trace;
use std::collections::HashSet;

pub fn hidden_singles(
    state: &mut GameState,
    candidates: &SetOfMoveCandidates,
) -> Result<StrategyMove, StrategyError> {
    let mut applied = Vec::new();

    for candidate in candidates.iter() {
        let value_candidates = candidate.value_hashset();
        let peer_values = state.get_peer_values(candidate.index);

        let mut difference: HashSet<_> = value_candidates.difference(&peer_values).collect();

        if difference.len() == 1 {
            let value = *difference.drain().next().unwrap();
            applied.push(Placement::new(value, candidate.index));
            trace!(
                "  * Hidden single {value} at index {index}",
                value = value,
                index = candidate.index
            );
        }
    }

    if applied.is_empty() {
        Ok(StrategyMove::None)
    } else {
        Ok(StrategyMove::Applied(applied))
    }
}
